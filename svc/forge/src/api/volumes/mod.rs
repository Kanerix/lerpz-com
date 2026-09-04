use crate::state::AppState;

use k8s_openapi::api::core::v1::PersistentVolumeClaim;
use kube::ResourceExt;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::resources;

mod create;
mod delete;
mod list;
mod read;

/// Label component value identifying a memory volume.
pub(crate) const COMPONENT: &str = "memory";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler, create::handler))
        .routes(routes!(read::handler, delete::handler))
}

/// A persistent memory volume attached to an agent.
#[derive(Debug, Serialize, ToSchema)]
pub struct MemoryVolume {
    /// Name of the underlying `PersistentVolumeClaim`
    pub name: String,
    /// Identifier of the agent this volume belongs to
    pub agent: Option<String>,
    /// Claim phase reported by the cluster (`Pending`, `Bound`, `Lost`)
    pub phase: Option<String>,
    /// Requested storage size, e.g. `10Gi`
    pub size: Option<String>,
    /// StorageClass backing the claim
    pub storage_class: Option<String>,
    /// When the claim was created
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PersistentVolumeClaim> for MemoryVolume {
    fn from(pvc: PersistentVolumeClaim) -> Self {
        let name = pvc.name_any();
        let agent = resources::agent_of(pvc.metadata.labels.as_ref());
        let created_at = resources::timestamp(pvc.metadata.creation_timestamp.as_ref());

        let spec = pvc.spec.as_ref();
        let size = spec
            .and_then(|spec| spec.resources.as_ref())
            .and_then(|resources| resources.requests.as_ref())
            .and_then(|requests| requests.get("storage"))
            .map(|quantity| quantity.0.clone());
        let storage_class = spec.and_then(|spec| spec.storage_class_name.clone());
        let phase = pvc.status.as_ref().and_then(|status| status.phase.clone());

        Self {
            name,
            agent,
            phase,
            size,
            storage_class,
            created_at,
        }
    }
}
