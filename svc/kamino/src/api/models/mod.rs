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

/// Provider family a model belongs to.
///
/// Mirrors the `model_family` Postgres enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "model_family", rename_all = "lowercase")]
pub enum ModelFamily {
    OpenAI,
    Anthropic,
    Google,
}

/// An AI model that can be routed to via Portkey.
#[derive(Debug, Serialize, ToSchema)]
pub struct Model {
    /// Unique model identifier.
    pub id: Uuid,
    /// Human-readable name shown in UIs (e.g. `GPT-4o`).
    pub display_name: String,
    /// Optional longer description of the model.
    pub description: Option<String>,
    /// Provider family the model belongs to.
    pub family: ModelFamily,
    /// Portkey deployment name used when routing requests.
    pub deployment_name: String,
    /// Portkey provider slug the deployment lives under.
    pub provider: String,
    /// Arbitrary provider/runtime settings as a JSON object.
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
