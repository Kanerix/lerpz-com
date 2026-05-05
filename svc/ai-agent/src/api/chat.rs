//! Chat handler for the AI agent HTTP API.

use std::sync::Arc;

use axum::http::HeaderMap;
use axum::{Json, extract::State};
use http::StatusCode;
use lerpz_axum::error::{HandlerError, HandlerErrorSchema, HandlerResult};
use lerpz_axum::middleware::azure::AzureAccessToken;
use rig::agent::Agent;
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::factory::AgentFactory;
use crate::oapi::AGENT_TAG;
use crate::state::AppState;

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ChatResponse {
    pub response: String,
}

#[utoipa::path(
    post,
    path = "/chat",
    tag = AGENT_TAG,
    summary = "Chat with the AI agent",
    description = "Send a plain-text message to the AI agent and receive a \
        response. The agent will automatically retrieve relevant context from \
        the knowledge base and invoke tools as needed.",
    request_body = ChatRequest,
    responses(
        (
            status = OK,
            description = "Agent replied successfully",
            body = ChatResponse
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error",
            body = HandlerErrorSchema
        ),
    )
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    _: AzureAccessToken,
    headers: HeaderMap,
    State(agent_factory): State<Arc<AgentFactory>>,
    Json(payload): Json<ChatRequest>,
) -> HandlerResult<Json<ChatResponse>> {
    if payload.message.is_empty() {
        return Err(HandlerError::new(
            StatusCode::BAD_REQUEST,
            "Message cannot be empty",
            "Please provide a message to send to the AI agent",
        ));
    }

    let bearer = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or_default()
        .to_owned();

    let agent = agent_factory.create(bearer);
    let message = payload.message.as_str();
    let response = agent.prompt(message).await?;

    Ok(Json(ChatResponse { response }))
}
