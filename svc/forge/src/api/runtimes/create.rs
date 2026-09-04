use std::collections::BTreeMap;

use axum::{Json, extract::State, http::StatusCode};
use k8s_openapi::{
    api::{
        apps::v1::{Deployment, DeploymentSpec},
        core::v1::{
            Container, EnvVar, PersistentVolumeClaimVolumeSource, PodSpec, PodTemplateSpec,
            ResourceRequirements, Volume, VolumeMount,
        },
    },
    apimachinery::pkg::{
        api::resource::Quantity,
        apis::meta::v1::{LabelSelector, ObjectMeta},
    },
};
use kube::api::PostParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{
    api::runtimes::{AgentRuntime, COMPONENT},
    config::CONFIG,
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
    /// Number of replicas. Defaults to `1`.
    replicas: Option<i32>,
    /// Mount the agent's memory volume. Defaults to `true`; the volume must
    /// already have been provisioned.
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
        rejected with a `409` rather than starting a second runtime.",
    request_body(
        content = CreateRuntimeRequest,
        description = "Runtime parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = CREATED,
            description = "The runtime was provisioned",
            body = AgentRuntime
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent identifier or resource quantity",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
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
    _token: AzureAccessToken,
    State(kube): State<KubeClient>,
    Json(body): Json<CreateRuntimeRequest>,
) -> HandlerResult<(StatusCode, Json<AgentRuntime>)> {
    resources::validate_agent(&body.agent)?;

    let name = resources::runtime_name(&body.agent);
    let image = body
        .image
        .unwrap_or_else(|| CONFIG.AGENT_RUNTIME_IMAGE.to_string());
    let replicas = body.replicas.unwrap_or(1);
    let labels = resources::labels(&body.agent, COMPONENT);

    let mount_memory = body.mount_memory.unwrap_or(true);
    let claim_name = resources::memory_volume_name(&body.agent);

    let (volumes, volume_mounts) = if mount_memory {
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

    tracing::info!(agent = %body.agent, %name, %image, %replicas, "provisioning agent runtime");

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(CONFIG.KUBE_NAMESPACE.to_string()),
            labels: Some(labels.clone()),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(replicas),
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
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

    let created = resources::runtime_api(kube)
        .create(&PostParams::default(), &deployment)
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?;

    Ok((StatusCode::CREATED, Json(created.into())))
}
