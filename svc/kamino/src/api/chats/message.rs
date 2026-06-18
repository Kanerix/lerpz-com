use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
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
use tokio_stream::Stream;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::chats::stream::start_completion_sse,
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct MessageRequest {
    /// The user's message text
    prompt: String,
    /// Reasoning level for reasoning-capable models (`none`, `minimal`, `low`,
    /// `medium`, `high` or `xhigh`). Unknown values fall back to `low`. Uses
    /// the model's default behaviour when omitted.
    reasoning: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/{id}",
    operation_id = "send_chat_message",
    tag = CHATS_TAG,
    summary = "Send a message in an existing chat",
    description = "Appends a new user message to the conversation and streams \
        the AI reply back via Server-Sent Events. Requires the conversation to \
        belong to the authenticated user. SSE events emitted: `reasoning` \
        (chain-of-thought chunk, reasoning models only), `message` (answer token \
        chunk) streamed until the response ends, `saved` (conversation UUID \
        confirming persistence, sent as the final event before the stream \
        closes), `error` (error message).",
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
                           reasoning (reasoning token chunk), \
                           message (token chunk), \
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
    let reasoning = body.reasoning;

    tracing::trace!(%conv_id, %user_id, "loading conversation");
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
            tracing::warn!(%conv_id, %user_id, "conversation not found");
            return Err(Problem::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested conversation was not found.",
            ));
        }
    };

    let model = conversation.model;

    tracing::trace!(%conv_id, "loading previous messages");
    let previous_messages = sqlx::query!(
        "SELECT role AS \"role: String\", content
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC",
        &conv_id,
    )
    .fetch_all(&database)
    .await?;

    tracing::trace!(%conv_id, "persisting user message");
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

    // Apply an explicit reasoning level when the client provides one; otherwise
    // let the model use its default behaviour.
    if let Some(level) = reasoning.as_deref() {
        request_builder.reasoning_effort(super::parse_reasoning_effort(level));
    }

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let sse_stream = start_completion_sse(openai, request, conv_id, database).await?;

    Ok(Sse::new(sse_stream))
}
