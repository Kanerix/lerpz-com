use std::convert::Infallible;

use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::{Sse, sse::Event},
};
use lerpz_ai::generation::{ChatMessage, ChatRequest as GenerationChatRequest};
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
pub struct EditLatestMessageRequest {
    /// The replacement text for the conversation's latest user message.
    prompt: String,
    /// Optional model override. Switches the conversation to this model for
    /// this and future messages. Uses the conversation's current model when
    /// omitted.
    #[serde(default)]
    model: Option<String>,
    /// Reasoning level for reasoning-capable models (`none`, `minimal`, `low`,
    /// `medium`, `high` or `xhigh`). Unknown values fall back to `low`. Uses
    /// the model's default behaviour when omitted.
    reasoning: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/{id}/messages/latest",
    operation_id = "edit_latest_chat_message",
    tag = CHATS_TAG,
    summary = "Edit the latest message in a chat",
    description = "Replaces the content of the conversation's most recent user \
        message, discards the assistant reply (and any later turns) that followed \
        it, then regenerates and streams a fresh reply via Server-Sent Events. \
        Only the latest message can be edited, since editing an earlier one would \
        require regenerating everything after it.\n\n\
        Events:\n\
        - `reasoning`: chain-of-thought chunk, reasoning models only\n\
        - `message`: answer token chunk\n\
        - `saved`: conversation UUID confirming persistence, sent last\n\
        - `error`: problem document describing the failure",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
    ),
    request_body(
        content = EditLatestMessageRequest,
        description = "Replacement message parameters",
        content_type = "application/json"
    ),
    responses(
        (
            status = OK,
            description = "SSE stream of the AI reply; see the endpoint \
                           description for the event sequence",
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
            status = CONFLICT,
            description = "The conversation has no user message to edit",
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
    Json(body): Json<EditLatestMessageRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let user_id = token.sub.as_str();
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

    let latest_user_id = sqlx::query_scalar!(
        "SELECT id FROM messages
        WHERE conversation_id = $1 AND role = 'user'
        ORDER BY id DESC
        LIMIT 1",
        &conv_id,
    )
    .fetch_optional(&database)
    .await?;

    let latest_user_id = match latest_user_id {
        Some(id) => id,
        None => {
            tracing::warn!(%conv_id, "no user message to edit");
            return Err(Problem::new(
                StatusCode::CONFLICT,
                "Conflict",
                "This conversation has no message to edit.",
            ));
        }
    };

    tracing::trace!(%conv_id, %latest_user_id, "editing latest message");
    let mut tx = database.begin().await?;
    sqlx::query!(
        "UPDATE messages SET content = $1 WHERE id = $2",
        &prompt,
        &latest_user_id,
    )
    .execute(&mut *tx)
    .await?;
    sqlx::query!(
        "DELETE FROM messages WHERE conversation_id = $1 AND id > $2",
        &conv_id,
        &latest_user_id,
    )
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    let model = match body.model {
        Some(m) if !m.trim().is_empty() => m,
        _ => conversation.model.clone(),
    };
    if model != conversation.model {
        tracing::trace!(%conv_id, %model, "switching conversation model");
        sqlx::query!(
            "UPDATE conversations SET model = $1 WHERE id = $2",
            &model,
            &conv_id,
        )
        .execute(&database)
        .await?;
    }

    let model_family = sqlx::query_scalar!(
        "SELECT family FROM models WHERE deployment_name = $1 LIMIT 1",
        &model,
    )
    .fetch_optional(&database)
    .await?;

    tracing::trace!(%conv_id, "loading messages");
    let previous_messages = sqlx::query!(
        "SELECT role AS \"role: String\", content
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC",
        &conv_id,
    )
    .fetch_all(&database)
    .await?;

    let messages = previous_messages
        .into_iter()
        .filter_map(|msg| match msg.role.as_str() {
            "user" => Some(ChatMessage::User(msg.content)),
            "assistant" => Some(ChatMessage::Assistant(msg.content)),
            _ => None,
        })
        .collect();

    let request = GenerationChatRequest {
        model,
        messages,
        reasoning,
        user: token.upn.clone(),
    };
    let sse_stream = start_completion_sse(openai, request, conv_id, database, model_family).await?;

    Ok(Sse::new(sse_stream))
}
