//! Chat handler for the AI agent HTTP API.

use std::sync::Arc;

use axum::http::HeaderMap;
use axum::response::Sse;
use axum::{Json, extract::State};
use http::StatusCode;
use lerpz_axum::middleware::azure::AzureAccessToken;
use lerpz_axum::problem::{HandlerResult, Problem, ProblemSchema};
use rig::agent::Agent;
use rig::completion::Prompt;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::factory::AgentFactory;
use crate::oapi::AGENT_TAG;
use crate::state::{AppState, DatabasePool};
use crate::stream::AgentStream;

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
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Internal server error",
            body = ProblemSchema
        ),
    )
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    _: AzureAccessToken,
    headers: HeaderMap,
    State(db): State<DatabasePool>,
    State(agent_factory): State<Arc<AgentFactory>>,
    Json(payload): Json<ChatRequest>,
) -> HandlerResult<Sse<AgentStream>> {
    if payload.message.is_empty() {
        return Err(Problem::new(
            StatusCode::BAD_REQUEST,
            "Message cannot be empty",
            "Please provide a message to send to the AI agent",
        ));
    }

    let bearer = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(Problem::unauthorized())?;

    let agent = agent_factory.create(bearer);
    let message = payload.message.as_str();
    let response = agent.prompt(message).await?;

    let agent_stream = AgentStream::new(db);

    Ok(Sse::new(agent_stream))
}
