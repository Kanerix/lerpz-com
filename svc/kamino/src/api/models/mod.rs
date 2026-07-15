use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod list;
mod read;
mod update;

/// Provider/runtime settings for a model, stored as a JSON object.
///
/// The object is open-ended; only the keys the backend understands are
/// documented here.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ModelSettings {
    /// Whether the model produces reasoning / chain-of-thought output that is
    /// streamed via the `reasoning` SSE event and stored alongside replies.
    #[serde(default)]
    pub reasoning: bool,
}

/// An AI model that can be routed to via Portkey.
#[derive(Debug, Serialize, ToSchema)]
pub struct Model {
    /// Unique model identifier.
    pub id: Uuid,
    /// Human-readable name shown in UIs.
    pub display_name: String,
    /// Optional longer description of the model.
    pub description: Option<String>,
    /// Provider family the model belongs to.
    pub family: String,
    /// Portkey deployment name used when routing requests.
    pub deployment_name: String,
    /// Portkey provider slug the deployment lives under.
    pub provider: String,
    /// Input/output modalities the model supports.
    pub modalities: Vec<String>,
    /// Provider/runtime settings as a JSON object.
    #[schema(value_type = ModelSettings)]
    pub settings: serde_json::Value,
    /// Timestamp of when the model was created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of the last update.
    pub updated_at: DateTime<Utc>,
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler, create::handler))
        .routes(routes!(read::handler, update::handler, delete::handler))
}
