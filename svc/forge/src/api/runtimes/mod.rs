use crate::state::AppState;

use k8s_openapi::api::apps::v1::Deployment;
use kube::ResourceExt;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{networking, resources};

mod authorize;
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
        .routes(routes!(authorize::handler))
}

/// A container runtime executing an agent.
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentRuntimeResponse {
    /// Name of the underlying `Deployment`
    pub name: String,
    /// Identifier of the agent this runtime executes
    pub agent: Option<String>,
    /// Runtime UUID, absent on legacy deployments
    pub runtime_id: Option<String>,
    /// Public runtime URL recorded at creation, without an API suffix
    pub base_url: Option<String>,
    /// Named HTTP port on the runtime container, if valid and recorded
    pub port: Option<u16>,
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
    /// Entra object ID of the caller that created the runtime, if recorded
    pub created_by_oid: Option<String>,
    /// Entra tenant ID of the creator, if recorded
    pub created_by_tenant_id: Option<String>,
    /// Ownership kind, currently `user`, if recorded
    pub owner_type: Option<String>,
    /// Entra object ID of the owner, if recorded
    pub owner_id: Option<String>,
    /// Entra tenant ID of the owner, if recorded
    pub owner_tenant_id: Option<String>,
}

impl From<Deployment> for AgentRuntimeResponse {
    fn from(deployment: Deployment) -> Self {
        let name = deployment.name_any();
        let agent = resources::agent_of(deployment.metadata.labels.as_ref());
        let runtime_id = deployment
            .labels()
            .get(networking::RUNTIME_ID_LABEL)
            .cloned();
        let base_url = deployment
            .annotations()
            .get(networking::BASE_URL_ANNOTATION)
            .cloned();
        let created_at = resources::timestamp(deployment.metadata.creation_timestamp.as_ref());
        let created_by_oid = deployment
            .labels()
            .get(resources::CREATED_BY_OID_LABEL)
            .cloned();
        let created_by_tenant_id = deployment
            .labels()
            .get(resources::CREATED_BY_TENANT_ID_LABEL)
            .cloned();

        let owner_type = deployment
            .labels()
            .get(resources::OWNER_TYPE_LABEL)
            .cloned();
        let owner_id = deployment.labels().get(resources::OWNER_ID_LABEL).cloned();
        let owner_tenant_id = deployment
            .labels()
            .get(resources::OWNER_TENANT_ID_LABEL)
            .cloned();

        let pod_spec = deployment
            .spec
            .as_ref()
            .and_then(|spec| spec.template.spec.as_ref());

        let port = pod_spec
            .and_then(|spec| {
                spec.containers
                    .iter()
                    .find(|container| container.name == COMPONENT)
            })
            .and_then(|container| container.ports.as_ref())
            .and_then(|ports| {
                ports
                    .iter()
                    .find(|port| port.name.as_deref() == Some("http"))
            })
            .and_then(|port| u16::try_from(port.container_port).ok())
            .filter(|port| *port > 0);

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
            runtime_id,
            base_url,
            port,
            image,
            replicas,
            ready_replicas,
            memory_volume,
            created_at,
            created_by_oid,
            created_by_tenant_id,
            owner_type,
            owner_id,
            owner_tenant_id,
        }
    }
}
