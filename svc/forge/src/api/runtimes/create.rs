use std::collections::BTreeMap;

use axum::{Json, extract::State, http::StatusCode};
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, ContainerPort, EnvVar, PersistentVolumeClaimVolumeSource, PodSpec,
            PodTemplateSpec, Probe, ResourceRequirements, TCPSocketAction, Volume, VolumeMount,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity,
        apis::meta::v1::{LabelSelector, ObjectMeta},
        util::intstr::IntOrString,
    },
};
use kube::api::{DeleteParams, PostParams, Preconditions};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    api::runtimes::{AgentRuntimeResponse, COMPONENT},
    config::CONFIG,
    networking,
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

/// Where an agent's persistent memory is mounted inside the container.
const MEMORY_MOUNT_PATH: &str = "/var/lib/lerpz/memory";

/// Name of the volume entry in the pod spec.
const MEMORY_VOLUME_NAME: &str = "memory";

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateRuntimeRequest {
    /// Identifier of the agent to run. Must be a DNS-1123 label (lowercase
    /// alphanumerics and `-`), at most 40 characters.
    agent: String,
    /// Container image to run. Falls back to the configured default image when
    /// omitted.
    image: Option<String>,
    /// HTTP port the application listens on. Defaults to AGENT_RUNTIME_PORT.
    /// The application must bind this port on 0.0.0.0; declaring it does not configure the image.
    #[schema(minimum = 1, maximum = 65535)]
    port: Option<u16>,
    /// Number of replicas. Defaults to `1`.
    replicas: Option<i32>,
    /// Mount the agent's memory volume. Defaults to `true`; the volume must
    /// already have been provisioned by the caller.
    mount_memory: Option<bool>,
    /// CPU limit as a Kubernetes quantity, e.g. `500m`.
    cpu_limit: Option<String>,
    /// Memory limit as a Kubernetes quantity, e.g. `512Mi`.
    memory_limit: Option<String>,
    /// Extra environment variables passed to the container.
    #[serde(default)]
    env: BTreeMap<String, String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_runtime",
    tag = RUNTIMES_TAG,
    summary = "Provision an agent runtime",
    description = "Creates the `Deployment` that executes an agent, optionally \
        mounting the agent's memory volume at `/var/lib/lerpz/memory`. The \
        deployment is named `agent-{agent}-runtime`, so repeat calls are \
        rejected with a `409` rather than starting a second runtime. The deployment \
        and its pods record the authenticated caller's Entra object and tenant IDs \
        as creator labels and set separate owner labels with `owner-type=user`. \
        Ownership is taken from the token, not the request body. If memory is \
        mounted, its volume must also be user-owned by the caller. A ClusterIP Service \
        and authenticated HTTPS ingress expose the runtime at the returned base_url. \
        The UUID path prefix is stripped before forwarding. Network resources are \
        garbage-collected with the deployment.",
    request_body(
        content = CreateRuntimeRequest,
        description = "Runtime parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = CREATED,
            description = "The runtime was provisioned",
            body = AgentRuntimeResponse
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent identifier, HTTP port or resource quantity",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token, or missing caller identity",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = FORBIDDEN,
            description = "Missing required delegated user permission, or cluster access was refused",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "The memory volume does not exist or is not accessible to the caller",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = CONFLICT,
            description = "The agent already has a runtime",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(kube): State<KubeClient>,
    Json(body): Json<CreateRuntimeRequest>,
) -> HandlerResult<(StatusCode, Json<AgentRuntimeResponse>)> {
    resources::validate_agent(&body.agent)?;
    let (object_id, tenant_id) = resources::caller_identity(&token)?;

    let port = body.port.unwrap_or(CONFIG.AGENT_RUNTIME_PORT.get());
    if port == 0 {
        return Err(Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid runtime port",
            "The runtime HTTP port must be between 1 and 65535.",
        ));
    }
    let runtime_id = Uuid::new_v4().to_string();
    let mut selector_labels = resources::labels(&body.agent, COMPONENT);
    selector_labels.insert(networking::RUNTIME_ID_LABEL.to_owned(), runtime_id.clone());
    let mut labels = selector_labels.clone();
    labels.extend(resources::creation_labels(&token)?);

    let name = resources::runtime_name(&body.agent);
    let image = body
        .image
        .unwrap_or_else(|| CONFIG.AGENT_RUNTIME_IMAGE.to_string());
    let replicas = body.replicas.unwrap_or(1);

    let mount_memory = body.mount_memory.unwrap_or(true);
    let claim_name = resources::memory_volume_name(&body.agent);

    let (volumes, volume_mounts) = if mount_memory {
        let claim = resources::volume_api(kube.clone())
            .get_opt(&claim_name)
            .await
            .map_err(|err| resources::kube_problem(err, "memory volume"))?
            .ok_or_else(|| resources::not_found("memory volume", &claim_name))?;

        if !resources::is_owned_by(
            claim.metadata.labels.as_ref(),
            &body.agent,
            object_id,
            tenant_id,
        ) {
            return Err(resources::not_found("memory volume", &claim_name));
        }

        (
            Some(vec![Volume {
                name: MEMORY_VOLUME_NAME.to_owned(),
                persistent_volume_claim: Some(PersistentVolumeClaimVolumeSource {
                    claim_name: claim_name.clone(),
                    read_only: Some(false),
                }),
                ..Default::default()
            }]),
            Some(vec![VolumeMount {
                name: MEMORY_VOLUME_NAME.to_owned(),
                mount_path: MEMORY_MOUNT_PATH.to_owned(),
                ..Default::default()
            }]),
        )
    } else {
        (None, None)
    };

    let env = body
        .env
        .into_iter()
        .map(|(name, value)| EnvVar {
            name,
            value: Some(value),
            ..Default::default()
        })
        .collect::<Vec<_>>();

    let mut limits = BTreeMap::new();
    if let Some(cpu) = body.cpu_limit {
        limits.insert("cpu".to_owned(), Quantity(cpu));
    }
    if let Some(memory) = body.memory_limit {
        limits.insert("memory".to_owned(), Quantity(memory));
    }

    tracing::info!(agent = %body.agent, %name, %runtime_id, %image, %replicas, %port, "provisioning agent runtime");

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(CONFIG.KUBE_NAMESPACE.to_string()),
            labels: Some(labels.clone()),
            annotations: Some(BTreeMap::from([(
                networking::BASE_URL_ANNOTATION.to_owned(),
                networking::base_url(&runtime_id),
            )])),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(replicas),
            selector: LabelSelector {
                match_labels: Some(selector_labels),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    // Agent containers run arbitrary workloads, so they get a
                    // ServiceAccount of their own rather than inheriting the one
                    // Forge uses to talk to the API server.
                    service_account_name: Some(CONFIG.AGENT_RUNTIME_SERVICE_ACCOUNT.to_string()),
                    automount_service_account_token: Some(false),
                    containers: vec![Container {
                        name: COMPONENT.to_owned(),
                        image: Some(image),
                        ports: Some(vec![ContainerPort {
                            name: Some("http".to_owned()),
                            container_port: i32::from(port),
                            ..Default::default()
                        }]),
                        readiness_probe: Some(Probe {
                            tcp_socket: Some(TCPSocketAction {
                                port: IntOrString::String("http".to_owned()),
                                ..Default::default()
                            }),
                            ..Default::default()
                        }),
                        env: (!env.is_empty()).then_some(env),
                        volume_mounts,
                        resources: (!limits.is_empty()).then(|| ResourceRequirements {
                            limits: Some(limits),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }],
                    volumes,
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    let api = resources::runtime_api(kube.clone());
    let created = api
        .create(&PostParams::default(), &deployment)
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?;

    if let Err(problem) = networking::provision(kube, &created, &runtime_id).await {
        if let Some(uid) = &created.metadata.uid {
            // Status updates must not prevent rollback of the deployment we just created.
            let params = DeleteParams {
                preconditions: Some(Preconditions {
                    uid: Some(uid.clone()),
                    resource_version: None,
                }),
                ..Default::default()
            };
            if let Err(err) = api.delete(&name, &params).await {
                tracing::error!(%name, %runtime_id, error = %err, "rolling back runtime networking failed");
                return Err(Problem::new(
                    StatusCode::BAD_GATEWAY,
                    "Runtime cleanup failed",
                    "Networking could not be provisioned and cleanup failed. Delete the runtime before retrying.",
                ).with_error(err));
            }
        }
        return Err(problem);
    }

    Ok((StatusCode::CREATED, Json(created.into())))
}
