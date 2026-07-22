//! Default chat completion streaming.
//!
//! Reasoning models routed through the gateway emit their chain-of-thought in
//! one of two shapes the typed `async-openai` stream drops: a flat
//! `reasoning_content` (a.k.a. `reasoning`) delta field, or Anthropic-style
//! incremental `content_blocks` carrying `thinking` deltas. We therefore use
//! the `byot` ("bring your own type") API with a minimal [`StreamChunk`] type
//! that preserves both.

use std::pin::Pin;

use async_openai::{
    Client, config::Config, error::OpenAIError, types::chat::CreateChatCompletionRequest,
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};

use crate::{generation::ChatStream, portkey::classify_error};

/// An event emitted while streaming a chat completion.
#[derive(Debug, Clone)]
pub enum ChatEvent {
    /// An incremental chain-of-thought / reasoning chunk.
    Reasoning(String),
    /// An incremental answer chunk.
    Message(String),
    /// The provider's content filter blocked the response.
    Filtered,
}

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
    /// Anthropic-style streaming delivers incremental reasoning and answer text
    /// as `content_blocks` instead of `reasoning_content`. We read the
    /// `thinking` deltas from here; the answer text is already mirrored in
    /// `content`, so we ignore the `text` blocks to avoid duplicating it.
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

/// Starts streaming a chat completion.
pub(super) async fn stream<C: Config>(
    client: &Client<C>,
    request: CreateChatCompletionRequest,
) -> Result<ChatStream, OpenAIError> {
    tracing::trace!(
        model = ?request.model,
        reasoning = ?request.reasoning_effort,
        "starting chat stream"
    );

    let stream = client
        .chat()
        .create_stream_byot::<_, StreamChunk>(request)
        .await?;

    Ok(chat_events(stream))
}

/// Adapts the raw chunk stream into a stream of [`ChatEvent`]s.
///
/// Reasoning and answer deltas are aggregated per chunk (across choices) before
/// being emitted, matching how the provider batches them.
fn chat_events(mut stream: ChunkStream) -> ChatStream {
    Box::pin(async_stream::stream! {
        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(chunk) => chunk,
                Err(err) => {
                    yield Err(classify_error(&err.to_string()));
                    break;
                }
            };

            let mut content = String::new();
            let mut reasoning = String::new();
            let mut filtered = false;

            for choice in &chunk.choices {
                if let Some(delta) = &choice.delta.reasoning_content {
                    reasoning.push_str(delta);
                }
                for block in &choice.delta.content_blocks {
                    if let Some(delta) = &block.delta.thinking {
                        reasoning.push_str(delta);
                    }
                }
                if let Some(delta) = &choice.delta.content {
                    content.push_str(delta);
                }
                if choice.finish_reason.as_deref() == Some("content_filter") {
                    filtered = true;
                }
            }

            if !reasoning.is_empty() {
                yield Ok(ChatEvent::Reasoning(reasoning));
            }
            if !content.is_empty() {
                yield Ok(ChatEvent::Message(content));
            }
            if filtered {
                yield Ok(ChatEvent::Filtered);
            }
        }
    })
}
