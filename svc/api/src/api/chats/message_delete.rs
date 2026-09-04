use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool},
};

#[utoipa::path(
    method(delete),
    path = "/{id}/messages/{message_id}",
    operation_id = "delete_chat_message",
    tag = CHATS_TAG,
    summary = "Delete a message and everything after it",
    description = "Permanently deletes the given message and all messages that \
        follow it in the conversation, keeping every earlier message intact. \
        Requires the conversation to belong to the authenticated user.",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
        ("message_id" = Uuid, Path, description = "ID of the first message to delete"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "Message and all following messages deleted"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Conversation or message not found, or does not belong to the authenticated user",
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
    Path((conv_id, message_id)): Path<(Uuid, Uuid)>,
    State(database): State<DatabasePool>,
) -> HandlerResult<StatusCode> {
    let user_id = token.sub;

    tracing::trace!(%conv_id, %user_id, "loading conversation");
    let conversation = sqlx::query_scalar!(
        "SELECT id FROM conversations WHERE id = $1 AND user_id = $2",
        &conv_id,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    if conversation.is_none() {
        tracing::warn!(%conv_id, %user_id, "conversation not found");
        return Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested conversation was not found.",
        ));
    }

    // `id` is a uuidv7, so it is monotonic with insertion order: deleting every
    // message with an `id` at or after the target removes the message itself and
    // all later turns while leaving earlier messages untouched. Existence is
    // checked via `rows_affected` below, so a missing message yields a 404.
    tracing::trace!(%conv_id, %message_id, "deleting message and everything after it");
    let result = sqlx::query!(
        "DELETE FROM messages WHERE conversation_id = $1 AND id >= $2",
        &conv_id,
        &message_id,
    )
    .execute(&database)
    .await?;

    if result.rows_affected() == 0 {
        tracing::warn!(%conv_id, %message_id, "message not found");
        return Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested message was not found.",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
