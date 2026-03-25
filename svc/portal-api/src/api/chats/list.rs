use axum::{Json, extract::State};
use chrono::NaiveDateTime;
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool},
};

#[derive(Debug, Serialize)]
pub struct Conversation {
    id: Uuid,
    title: Option<String>,
    model: String,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
}

#[utoipa::path(
    method(get),
    path = "/",
    tag = CHATS_TAG,
    summary = "Get a list of chats",
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
