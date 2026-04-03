use axum::Json;
use chrono::NaiveDateTime;
use lerpz_axum::error::{HandlerErrorSchema, HandlerResult};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::oapi::CHATS_TAG;

use crate::state::AppState;

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
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
