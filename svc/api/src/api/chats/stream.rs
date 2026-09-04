//! Shared Server-Sent Events streaming logic for chat completions.
//!
//! Both creating a chat and sending a message in an existing chat stream the
//! assistant reply back the same way, so the streaming loop lives here. The
//! provider-specific mechanics of talking to the model live in the `lerpz-ai`
//! crate; this module maps its [`ChatEvent`]s onto SSE events and persists the
//! assembled reply.

use std::convert::Infallible;

use async_openai::types::chat::CreateChatCompletionRequest;
use axum::response::sse::Event;
use lerpz_ai::generation::{ChatEvent, ChatStream, Family};
use lerpz_axum::problem::HandlerResult;
use tokio_stream::{Stream, StreamExt as _};
use uuid::Uuid;

use crate::state::{DatabasePool, OpenAI};

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
    let family = Family::from_name(model_family.as_deref());
    let stream = family.chat_stream(openai.as_ref(), request).await?;

    Ok(completion_sse(stream, conv_id, database, model_family))
}

fn completion_sse(
    mut stream: ChatStream,
    conv_id: Uuid,
    database: DatabasePool,
    model_family: Option<String>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        let mut content_buf = String::new();
        let mut reasoning_buf = String::new();

        while let Some(event) = stream.next().await {
            match event {
                Err(upstream) => {
                    if upstream.is_user() {
                        tracing::warn!(%conv_id, reason = %upstream.message, "chat completion rejected by provider");
                    } else {
                        tracing::error!(%conv_id, "chat completion failed: {}", upstream.message);
                    }
                    yield Ok(Event::default().event("error").data(upstream.message));
                    break;
                }
                Ok(ChatEvent::Reasoning(reasoning)) => {
                    reasoning_buf.push_str(&reasoning);
                    yield Ok(Event::default().event("reasoning").data(reasoning));
                }
                Ok(ChatEvent::Message(content)) => {
                    content_buf.push_str(&content);
                    yield Ok(Event::default().event("message").data(content));
                }
                Ok(ChatEvent::Filtered) => {
                    tracing::warn!(%conv_id, "content filter triggered");
                    yield Ok(Event::default()
                        .event("error")
                        .data("content filter triggered"));
                }
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
