use axum::{Json, extract::State};
use chrono::{NaiveDateTime, Utc};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool},
};

#[derive(Debug, Serialize, ToSchema)]
pub struct Conversation {
    /// Unique conversation identifier
    id: Uuid,
    /// Conversation title (auto-generated from the first prompt if not provided)
    title: Option<String>,
    /// AI model used for this conversation
    model: String,
    /// Timestamp of when the conversation was created
    created_at: Option<chrono::DateTime<Utc>>,
    /// Timestamp of the last message in the conversation
    updated_at: Option<chrono::DateTime<Utc>>,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_chats",
    tag = CHATS_TAG,
    summary = "Get a list of chats",
    description = "Returns a list of the authenticated user's conversations ordered by most recently updated.",
    responses(
        (
            status = OK,
            description = "List of conversations",
            body = Vec<Conversation>
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
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
    State(database): State<DatabasePool>,
) -> HandlerResult<Json<Vec<Conversation>>> {
    let user_id = token.sub;

    let conversations = sqlx::query_as!(
        Conversation,
        "SELECT id, title, model, created_at, updated_at
        FROM conversations
        WHERE user_id = $1 ORDER BY updated_at DESC",
        &user_id,
    )
    .fetch_all(&database)
    .await?;

    Ok(Json(conversations))
}
