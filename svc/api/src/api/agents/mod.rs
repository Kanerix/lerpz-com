use crate::state::AppState;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use lerpz_axum::problem::{HandlerResult, Problem};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::forge::RuntimeResponse;

mod create;
mod delete;
mod list;
pub(super) mod memory;
mod read;

/// The public view of a private agent.
#[derive(Serialize, ToSchema)]
pub struct AgentResponse {
    name: String,
    status: AgentStatus,
    /// Whether the agent uses persistent memory.
    memory: bool,
    /// Authenticated application URL, when available. This does not guarantee readiness.
    url: Option<String>,
    created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentStatus {
    Ready,
    NotReady,
    Stopped,
    Unknown,
}

impl AgentResponse {
    fn from_runtime(runtime: RuntimeResponse) -> Self {
        let status = match (runtime.replicas, runtime.ready_replicas) {
            (Some(0), _) => AgentStatus::Stopped,
            (Some(desired), Some(ready)) if desired > 0 && ready >= desired => AgentStatus::Ready,
            (Some(desired), _) if desired > 0 => AgentStatus::NotReady,
            _ => AgentStatus::Unknown,
        };
        Self {
            name: runtime.agent,
            status,
            memory: runtime.memory_volume.is_some(),
            url: runtime.base_url,
            created_at: runtime.created_at,
        }
    }
}

fn validate_name(name: &str) -> HandlerResult<()> {
    if name.is_empty()
        || name.len() > 40
        || name.starts_with('-')
        || name.ends_with('-')
        || !name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
    {
        tracing::warn!(
            status = 400,
            reason = "invalid_name",
            "rejecting agent request"
        );
        return Err(Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid agent name",
            "Use 1 to 40 lowercase letters, digits or hyphens, starting and ending with a letter or digit.",
        ));
    }
    tracing::Span::current().record("agent_name", name);
    Ok(())
}

fn not_found() -> Problem {
    tracing::debug!(status = 404, "agent is absent or inaccessible");
    Problem::new(
        StatusCode::NOT_FOUND,
        "Agent not found",
        "The agent does not exist or is not accessible to you.",
    )
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler, create::handler))
        .routes(routes!(read::handler, delete::handler))
}
