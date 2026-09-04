use crate::state::AppState;

use k8s_openapi::api::apps::v1::Deployment;
use kube::ResourceExt;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::resources;

mod create;
mod delete;
mod list;
mod read;

/// Label component value identifying an agent runtime.
pub(crate) const COMPONENT: &str = "runtime";

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler, create::handler))
        .routes(routes!(read::handler, delete::handler))
}

/// A container runtime executing an agent.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentRuntime {
    /// Name of the underlying `Deployment`
    pub name: String,
    /// Identifier of the agent this runtime executes
    pub agent: Option<String>,
    /// Container image the runtime is running
    pub image: Option<String>,
    /// Desired number of replicas
    pub replicas: Option<i32>,
    /// Replicas currently reporting ready
    pub ready_replicas: Option<i32>,
    /// Memory volume mounted into the runtime, if any
    pub memory_volume: Option<String>,
    /// When the runtime was created
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<Deployment> for AgentRuntime {
    fn from(deployment: Deployment) -> Self {
        let name = deployment.name_any();
        let agent = resources::agent_of(deployment.metadata.labels.as_ref());
        let created_at = resources::timestamp(deployment.metadata.creation_timestamp.as_ref());

        let pod_spec = deployment
            .spec
            .as_ref()
            .and_then(|spec| spec.template.spec.as_ref());

        let image = pod_spec
            .and_then(|spec| spec.containers.first())
            .and_then(|container| container.image.clone());

        let memory_volume = pod_spec
            .and_then(|spec| spec.volumes.as_ref())
            .and_then(|volumes| volumes.first())
            .and_then(|volume| volume.persistent_volume_claim.as_ref())
            .map(|claim| claim.claim_name.clone());

        let replicas = deployment.spec.as_ref().and_then(|spec| spec.replicas);
        let ready_replicas = deployment
            .status
            .as_ref()
            .and_then(|status| status.ready_replicas);

        Self {
            name,
            agent,
            image,
            replicas,
            ready_replicas,
            memory_volume,
            created_at,
        }
    }
}
