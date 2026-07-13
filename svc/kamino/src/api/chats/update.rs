use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::chats::list::Conversation,
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateChatRequest {
    /// Whether the conversation should be archived (`true`) or restored
    /// (`false`).
    archived: bool,
}

#[utoipa::path(
    method(patch),
    path = "/{id}",
    operation_id = "update_chat",
    tag = CHATS_TAG,
    summary = "Update a chat",
    description = "Updates mutable fields on a conversation owned by the \
        authenticated user. Currently supports archiving and unarchiving.",
    params(
        ("id" = Uuid, Path, description = "Conversation ID"),
    ),
    request_body(
        content = UpdateChatRequest,
        description = "Fields to update on the conversation",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "The updated conversation",
            body = Conversation
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
    Json(body): Json<UpdateChatRequest>,
) -> HandlerResult<Json<Conversation>> {
    let user_id = token.sub;

    let updated = sqlx::query_as!(
        Conversation,
        "UPDATE conversations
        SET archived = $1
        WHERE id = $2 AND user_id = $3
        RETURNING id, title, model, archived, created_at, updated_at",
        body.archived,
        &conv_id,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    match updated {
        Some(conversation) => Ok(Json(conversation)),
        None => Err(Problem::new(
            StatusCode::NOT_FOUND,
            "Not Found",
            "The requested conversation was not found.",
        )),
    }
}
