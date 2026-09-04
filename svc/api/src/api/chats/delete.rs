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
    path = "/{id}",
    operation_id = "delete_chat",
    tag = CHATS_TAG,
    summary = "Delete a chat",
    description = "Permanently deletes a conversation and all of its messages. \
        Requires the conversation to belong to the authenticated user.",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "Conversation deleted"
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
    State(database): State<DatabasePool>,
) -> HandlerResult<StatusCode> {
    let user_id = token.sub;

    // Messages are removed automatically via the `ON DELETE CASCADE` foreign key
    // on `messages.conversation_id`.
    let result = sqlx::query!(
        "DELETE FROM conversations WHERE id = $1 AND user_id = $2",
        &conv_id,
        &user_id,
    )
    .execute(&database)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested conversation was not found.",
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
