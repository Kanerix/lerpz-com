//! Shared Server-Sent Events streaming logic for chat completions.
//!
//! Both creating a chat and sending a message in an existing chat stream the
//! assistant reply back the same way, so the streaming loop lives here. The
//! provider-specific mechanics of talking to the model live in the `lerpz-ai`
//! crate; this module maps its [`ChatEvent`]s onto SSE events and persists the
//! assembled reply.

use std::convert::Infallible;

use async_openai::types::chat::CreateChatCompletionRequest;
use axum::{http::StatusCode, response::sse::Event};
use lerpz_ai::generation::{ChatEvent, ChatStream, Family};
use lerpz_axum::problem::{HandlerResult, Problem};
use tokio_stream::{Stream, StreamExt as _};
use uuid::Uuid;

use crate::state::{DatabasePool, OpenAI};

/// Starts streaming a chat completion and returns an SSE stream of the reply.
///
/// The returned stream emits the following events:
/// - `reasoning`: incremental reasoning token chunk,
/// - `message`: incremental answer token chunk,
/// - `error`: a problem document describing why the reply stopped,
/// - `saved`: the conversation UUID, sent once the reply has been persisted.
///
/// The assistant message (answer plus any accumulated reasoning) is persisted
/// once the upstream stream completes. A stream that produced no answer, such
/// as one cut short by an upstream failure or a content filter, is not
/// persisted and ends without a `saved` event.
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
                    let problem: Problem = if upstream.is_user() {
                        Problem::new(
                            StatusCode::BAD_REQUEST,
                            "Chat completion rejected",
                            upstream.message.clone(),
                        )
                    } else {
                        Problem::new(
                            StatusCode::BAD_GATEWAY,
                            "Chat completion failed",
                            "The model provider could not complete this reply.",
                        )
                    };
                    yield Ok(problem.with_error(upstream).into_event());
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
                    tracing::warn!(%conv_id, "provider filters model output");
                    let problem: Problem = Problem::new(
                        StatusCode::BAD_REQUEST,
                        "Content filter triggered",
                        "The model provider blocked this reply.",
                    );
                    yield Ok(problem.into_event());
                }
            }
        }

        // An upstream failure or a content filter can end the stream before a
        // single token arrives. Persisting that would leave a blank assistant
        // message in the conversation, so there is nothing to save and nothing
        // to confirm.
        if content_buf.trim().is_empty() {
            tracing::debug!(%conv_id, "discarding assistant message with no content");
            return;
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
                tracing::trace!(%conv_id, "confirming saved assistant message");
                yield Ok(Event::default().event("saved").data(conv_id.to_string()));
            }
            Err(err) => {
                let problem: Problem = Problem::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Reply not saved",
                    "The reply was generated but could not be stored.",
                );
                yield Ok(problem.with_error(err).into_event());
            }
        }
    }
}
