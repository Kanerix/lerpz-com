//! Shared Server-Sent Events streaming logic for chat completions.
//!
//! Both creating a chat and sending a message in an existing chat stream the
//! assistant reply back the same way, so the streaming loop lives here.
//!
//! Reasoning models routed through Portkey emit their chain-of-thought in one
//! of two shapes the typed `async-openai` stream drops: a flat
//! `reasoning_content` (a.k.a. `reasoning`) delta field, or Anthropic-style
//! incremental `content_blocks` carrying `thinking` deltas. We therefore use
//! the `byot` ("bring your own type") API with a minimal [`StreamChunk`] type
//! that preserves both.

use std::convert::Infallible;
use std::pin::Pin;

use async_openai::error::OpenAIError;
use async_openai::types::chat::CreateChatCompletionRequest;
use axum::response::sse::Event;
use lerpz_axum::problem::HandlerResult;
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};
use uuid::Uuid;

use crate::state::{DatabasePool, OpenAI};

/// A minimal view of a streamed chat completion chunk.
///
/// Unknown fields are ignored, so this stays forward-compatible with whatever
/// else the provider includes in each chunk.
#[derive(Debug, Deserialize)]
struct StreamChunk {
    #[serde(default)]
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    #[serde(default)]
    delta: StreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct StreamDelta {
    /// Incremental answer tokens.
    #[serde(default)]
    content: Option<String>,
    /// Incremental reasoning / chain-of-thought tokens. Providers disagree on
    /// the field name, so accept both `reasoning_content` and `reasoning`.
    #[serde(default, alias = "reasoning")]
    reasoning_content: Option<String>,
    /// Anthropic-style streaming (via Portkey) delivers incremental reasoning
    /// and answer text as `content_blocks` instead of `reasoning_content`. We
    /// read the `thinking` deltas from here; the answer text is already
    /// mirrored in `content`, so we ignore the `text` blocks to avoid
    /// duplicating it.
    #[serde(default)]
    content_blocks: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(default)]
    delta: ContentBlockDelta,
}

#[derive(Debug, Default, Deserialize)]
struct ContentBlockDelta {
    /// Incremental reasoning / chain-of-thought tokens.
    #[serde(default)]
    thinking: Option<String>,
}

type ChunkStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, OpenAIError>> + Send>>;

/// Starts streaming a chat completion and returns an SSE stream of the reply.
///
/// The returned stream emits the following events:
/// - `reasoning`: incremental reasoning token chunk,
/// - `message`: incremental answer token chunk,
/// - `error`: an error message,
/// - `saved`: the conversation UUID, sent once the reply has been persisted.
///
/// The assistant message (answer plus any accumulated reasoning) is persisted
/// once the upstream stream completes.
pub(super) async fn start_completion_sse(
    openai: OpenAI,
    request: CreateChatCompletionRequest,
    conv_id: Uuid,
    database: DatabasePool,
    model_family: Option<String>,
) -> HandlerResult<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::trace!(
        %conv_id,
        model = ?request.model,
        reasoning = ?request.reasoning_effort,
        "starting chat stream"
    );

    let stream = openai
        .chat()
        .create_stream_byot::<_, StreamChunk>(request)
        .await?;

    Ok(completion_sse(stream, conv_id, database, model_family))
}

fn completion_sse(
    mut stream: ChunkStream,
    conv_id: Uuid,
    database: DatabasePool,
    model_family: Option<String>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        let mut content_buf = String::new();
        let mut reasoning_buf = String::new();

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(err) => {
                    tracing::error!("{err}");
                    yield Ok(Event::default()
                        .event("error")
                        .data(lerpz_portkey::humanize_error(&err.to_string())));
                    break;
                }
            };

            let mut content = String::new();
            let mut reasoning = String::new();
            let mut filtered = false;

            for choice in &chunk.choices {
                if let Some(ref delta) = choice.delta.reasoning_content {
                    reasoning.push_str(delta);
                }
                for block in &choice.delta.content_blocks {
                    if let Some(ref delta) = block.delta.thinking {
                        reasoning.push_str(delta);
                    }
                }
                if let Some(ref delta) = choice.delta.content {
                    content.push_str(delta);
                }
                if choice.finish_reason.as_deref() == Some("content_filter") {
                    tracing::warn!(%conv_id, "content filter triggered");
                    filtered = true;
                }
            }

            if !reasoning.is_empty() {
                reasoning_buf.push_str(&reasoning);
                yield Ok(Event::default().event("reasoning").data(reasoning));
            }

            if !content.is_empty() {
                content_buf.push_str(&content);
                yield Ok(Event::default().event("message").data(content));
            }

            if filtered {
                yield Ok(Event::default()
                    .event("error")
                    .data("content filter triggered"));
            }
        }

        tracing::trace!(%conv_id, "persisting assistant message");
        let reasoning = (!reasoning_buf.is_empty()).then_some(reasoning_buf);
        let result = sqlx::query!(
            "INSERT INTO messages (conversation_id, role, content, reasoning, model_family)
            VALUES ($1, 'assistant', $2, $3, $4)",
            &conv_id,
            &content_buf,
            reasoning.as_deref(),
            model_family.as_deref(),
        )
        .execute(&database)
        .await;

        match result {
            Ok(_) => {
                tracing::trace!(%conv_id, "saved assistant message");
                yield Ok(Event::default().event("saved").data(conv_id.to_string()));
            }
            Err(err) => {
                tracing::error!("failed to save assistant message: {err}");
                yield Ok(Event::default()
                    .event("error")
                    .data(format!("failed to save message: {err}")));
            }
        }
    }
}
