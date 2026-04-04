use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::NaiveDateTime;
use lerpz_axum::{
    error::{HandlerError, HandlerErrorSchema, HandlerResult},
    middleware::azure::AzureAccessToken,
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool},
};

/// A single message within a conversation.
#[derive(Debug, Serialize, ToSchema)]
pub struct ConversationMessage {
    /// Unique message identifier.
    id: Uuid,
    /// Message author: `"user"` or `"assistant"`.
    role: String,
    /// Raw message text.
    content: String,
    /// Timestamp when the message was created.
    created_at: NaiveDateTime,
}

/// A conversation together with all its messages.
#[derive(Debug, Serialize, ToSchema)]
pub struct ConversationDetail {
    /// Unique conversation identifier.
    id: Uuid,
    /// Conversation title.
    title: Option<String>,
    /// AI model used for this conversation.
    model: String,
    /// All messages in chronological order.
    messages: Vec<ConversationMessage>,
    /// Timestamp when the conversation was created.
    created_at: NaiveDateTime,
    /// Timestamp of the last update.
    updated_at: NaiveDateTime,
}

#[utoipa::path(
    method(get),
    path = "/{id}",
    operation_id = "get_chat",
    tag = CHATS_TAG,
    summary = "Get a specific chat",
    description = "Returns a conversation and all its messages for the authenticated user.",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
    ),
    responses(
        (
            status = OK,
            description = "Conversation detail with messages",
            body = ConversationDetail
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Conversation not found or does not belong to the authenticated user",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    Path(conv_id): Path<Uuid>,
    State(database): State<DatabasePool>,
) -> HandlerResult<Json<ConversationDetail>> {
    let user_id = token.sub;

    let conversation = sqlx::query!(
        "SELECT id, title, model, created_at, updated_at
        FROM conversations
        WHERE id = $1 AND user_id = $2",
        &conv_id,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    let conversation = match conversation {
        Some(c) => c,
        None => {
            return Err(HandlerError::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested conversation was not found.",
            ));
        }
    };

    let message_rows = sqlx::query!(
        "SELECT id, role AS \"role: String\", content, created_at
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC",
        &conv_id,
    )
    .fetch_all(&database)
    .await?;

    let messages = message_rows
        .into_iter()
        .map(|r| ConversationMessage {
            id: r.id,
            role: r.role,
            content: r.content,
            created_at: r.created_at.unwrap_or_default(),
        })
        .collect();

    Ok(Json(ConversationDetail {
        id: conversation.id,
        title: conversation.title,
        model: conversation.model,
        messages,
        created_at: conversation.created_at.unwrap_or_default(),
        updated_at: conversation.updated_at.unwrap_or_default(),
    }))
}