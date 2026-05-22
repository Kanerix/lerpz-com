use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, FinishReason,
};
use axum::{
    Json,
    extract::State,
    response::{Sse, sse::Event},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};
use utoipa::ToSchema;

use crate::{
    config::CONFIG,
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    /// Optional model override (uses the configured default when omitted)
    model: Option<String>,
    /// The user's first message / opening prompt
    prompt: String,
    /// Optional conversation title; auto-generated from the prompt when omitted
    title: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_chat",
    tag = CHATS_TAG,
    summary = "Create a new chat",
    description = "Creates a new conversation, streams the AI reply back via Server-Sent Events. \
        The first event is `conversation_created` with the new conversation ID; \
        subsequent `message` events carry incremental token chunks; \
        a `done` event signals the final chunk; \
        `saved` confirms the reply was persisted; \
        `error` is emitted on failures.",
    request_body(
        content = ChatRequest,
        description = "Chat creation parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "SSE stream of AI response chunks. Events: \
                conversation_created (conversation UUID), \
                message (token chunk), \
                done (final token chunk), \
                saved (conversation UUID), \
                error (error message)",
            content_type = "text/event-stream",
            body = String,
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(openai): State<OpenAI>,
    State(database): State<DatabasePool>,
    Json(body): Json<ChatRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let model = body
        .model
        .as_deref()
        .unwrap_or(&CONFIG.DEFAULT_COMPLETIONS_MODEL);
    let user_id = token.sub;
    let prompt = body.prompt;
    let title = body.title.unwrap_or_else(|| truncate_title(&prompt, 100));

    let conv_id = sqlx::query_scalar!(
        "INSERT INTO conversations (user_id, title, model)
        VALUES ($1, $2, $3)
        RETURNING id",
        &user_id,
        &title,
        &model
    )
    .fetch_one(&database)
    .await?;

    sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content)
        VALUES ($1, 'user', $2)",
        &conv_id,
        &prompt,
    )
    .execute(&database)
    .await?;

    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .max_tokens(2048u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let mut stream = openai.chat().create_stream(request).await?;

    let sse_stream = async_stream::stream! {
        yield Ok(Event::default()
            .event("conversation_created")
            .data(conv_id.to_string()));

        let mut assistant_buf = String::new();
        let mut completed = false;

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(err) => {
                    yield Ok(Event::default()
                        .event("error")
                        .data(format!("stream error: {err}")));
                    continue;
                }
            };

            let mut buf = String::new();
            let mut finished = false;

            for choice in &chunk.choices {
                if let Some(ref delta) = choice.delta.content {
                    buf.push_str(delta);
                }

                if let Some(finish_reason) = choice.finish_reason {
                    match finish_reason {
                        FinishReason::Stop => {
                            completed = true;
                            finished = true;
                        }
                        FinishReason::ContentFilter => {
                            yield Ok(Event::default()
                                .event("error")
                                .data("content filter triggered"));
                            break;
                        }
                        _ => {}
                    }
                }
            }

            assistant_buf.push_str(&buf);

            if finished {
                yield Ok(Event::default().event("done").data(buf));
            } else {
                yield Ok(Event::default().event("message").data(buf));
            }
        }

        if !completed {
            yield Ok(Event::default()
                .event("error")
                .data("stream ended without completion"));
            return;
        }

        let result = sqlx::query!(
            "INSERT INTO messages (conversation_id, role, content)
            VALUES ($1, 'assistant', $2)",
            &conv_id,
            &assistant_buf,
        )
        .execute(&database)
        .await;

        match result {
            Ok(_) => yield Ok(Event::default().event("saved").data(conv_id.to_string())),
            Err(err) => {
                tracing::error!("failed to save assistant message: {err}");
                yield Ok(Event::default()
                    .event("error")
                    .data(format!("failed to save message: {err}")));
            }
        }
    };

    Ok(Sse::new(sse_stream))
}

#[inline]
fn truncate_title(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect::<String>()
}
