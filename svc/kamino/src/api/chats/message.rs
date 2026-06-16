use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, FinishReason,
};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::{Sse, sse::Event},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct MessageRequest {
    /// The user's message text
    prompt: String,
}

#[utoipa::path(
    method(post),
    path = "/{id}",
    operation_id = "send_chat_message",
    tag = CHATS_TAG,
    summary = "Send a message in an existing chat",
    description = "Appends a new user message to the conversation and streams \
        the AI reply back via Server-Sent Events. Requires the conversation to \
        belong to the authenticated user. SSE events emitted: `message` (token \
        chunk), `done` (completion signal carrying the full assembled reply), \
        `saved` (conversation UUID confirming persistence), `error` (error message).",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
    ),
    request_body(
        content = MessageRequest,
        description = "Message parameters",
        content_type = "application/json"
    ),
    responses(
        (
            status = OK,
            description = "SSE stream of AI response chunks. Events: \
                           message (token chunk), \
                           done (full assembled reply), \
                           saved (conversation UUID), \
                           error (error message)",
            content_type = "text/event-stream",
            body = String
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Conversation not found or does not belong to the authenticated user",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    Path(conv_id): Path<Uuid>,
    State(openai): State<OpenAI>,
    State(database): State<DatabasePool>,
    Json(body): Json<MessageRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let user_id = token.sub;
    let prompt = body.prompt;

    let conversation = sqlx::query!(
        "SELECT id, model FROM conversations WHERE id = $1 AND user_id = $2",
        &conv_id,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    let conversation = match conversation {
        Some(c) => c,
        None => {
            return Err(Problem::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested conversation was not found.",
            ));
        }
    };

    let model = conversation.model;

    let previous_messages = sqlx::query!(
        "SELECT role AS \"role: String\", content
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC",
        &conv_id,
    )
    .fetch_all(&database)
    .await?;

    sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content)
        VALUES ($1, 'user', $2)",
        &conv_id,
        &prompt,
    )
    .execute(&database)
    .await?;

    let mut messages: Vec<async_openai::types::chat::ChatCompletionRequestMessage> = Vec::new();

    for msg in &previous_messages {
        match msg.role.as_str() {
            "user" => {
                messages.push(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(msg.content.clone())
                        .build()?
                        .into(),
                );
            }
            "assistant" => {
                messages.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(msg.content.clone())
                        .build()?
                        .into(),
                );
            }
            _ => {}
        }
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    );

    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .model(&model)
        .messages(messages)
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let mut stream = openai.chat().create_stream(request).await?;

    let sse_stream = async_stream::stream! {
        let mut assistant_buf = String::new();

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
                        FinishReason::Stop => { finished = true; }
                        FinishReason::ContentFilter => {
                            yield Ok(Event::default()
                                .event("error")
                                .data("content filter triggered"));
                            continue;
                        }
                        _ => {}
                    }
                }
            }

            assistant_buf.push_str(&buf);

            if !finished {
                yield Ok(Event::default().event("message").data(buf));
            }
        }

        yield Ok(Event::default().event("done").data(&assistant_buf));

        let result = sqlx::query!(
            "INSERT INTO messages (conversation_id, role, content) VALUES ($1, 'assistant', $2)",
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
