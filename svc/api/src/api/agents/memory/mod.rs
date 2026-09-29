use axum::http::StatusCode;
use lerpz_axum::problem::Problem;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::validate_name;
use crate::{forge::VolumeResponse, state::AppState};

mod delete;
mod list;
mod read;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler))
        .routes(routes!(read::handler, delete::handler))
}

/// Persistent agent memory, retained when the agent is deleted.
#[derive(Serialize, ToSchema)]
pub struct AgentMemoryResponse {
    /// Name of the agent this memory belongs to, even after the agent is deleted.
    pub agent_name: String,
    pub status: AgentMemoryStatus,
    /// Whether the matching agent has this memory mounted.
    pub in_use: bool,
    /// When the memory was created, if known.
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Availability of an agent's persistent memory.
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentMemoryStatus {
    Pending,
    Available,
    Unavailable,
    Unknown,
}

impl AgentMemoryResponse {
    fn from_volume(volume: VolumeResponse, in_use: bool) -> Self {
        let status = match volume.phase.as_deref() {
            Some("Pending") => AgentMemoryStatus::Pending,
            Some("Bound") => AgentMemoryStatus::Available,
            Some("Lost") => AgentMemoryStatus::Unavailable,
            _ => AgentMemoryStatus::Unknown,
        };
        Self {
            agent_name: volume.agent,
            status,
            in_use,
            created_at: volume.created_at,
        }
    }
}

fn not_found() -> Problem {
    tracing::debug!(status = 404, "agent memory is absent or inaccessible");
    Problem::new(
        StatusCode::NOT_FOUND,
        "Agent memory not found",
        "The requested agent memory does not exist or is not accessible to the caller.",
    )
}
