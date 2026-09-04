use axum::{Json, extract::State, http::StatusCode};
use k8s_openapi::{
    api::core::v1::{PersistentVolumeClaim, PersistentVolumeClaimSpec, VolumeResourceRequirements},
    apimachinery::pkg::{api::resource::Quantity, apis::meta::v1::ObjectMeta},
};
use kube::api::PostParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use std::collections::BTreeMap;
use utoipa::ToSchema;

use crate::{
    api::volumes::{COMPONENT, MemoryVolume},
    config::CONFIG,
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateVolumeRequest {
    /// Identifier of the agent the volume belongs to. Must be a DNS-1123 label
    /// (lowercase alphanumerics and `-`), at most 40 characters.
    agent: String,
    /// Requested size as a Kubernetes quantity, e.g. `10Gi`. Falls back to the
    /// configured default when omitted.
    size: Option<String>,
    /// StorageClass to provision from. Falls back to the configured default
    /// when omitted.
    storage_class: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_volume",
    tag = VOLUMES_TAG,
    summary = "Provision a memory volume",
    description = "Creates the `PersistentVolumeClaim` backing an agent's \
        persistent memory. The claim is named `agent-{agent}-memory`, so an \
        agent has exactly one memory volume and repeat calls are rejected with \
        a `409` rather than silently provisioning a second volume.",
    request_body(
        content = CreateVolumeRequest,
        description = "Memory volume parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = CREATED,
            description = "The memory volume was provisioned",
            body = MemoryVolume
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent identifier or size",
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
            description = "The agent already has a memory volume",
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
    Json(body): Json<CreateVolumeRequest>,
) -> HandlerResult<(StatusCode, Json<MemoryVolume>)> {
    resources::validate_agent(&body.agent)?;

    let name = resources::memory_volume_name(&body.agent);
    let size = body
        .size
        .unwrap_or_else(|| CONFIG.AGENT_MEMORY_DEFAULT_SIZE.to_string());
    let storage_class = body
        .storage_class
        .unwrap_or_else(|| CONFIG.AGENT_MEMORY_STORAGE_CLASS.to_string());

    tracing::info!(agent = %body.agent, %name, %size, "provisioning memory volume");

    let claim = PersistentVolumeClaim {
        metadata: ObjectMeta {
            name: Some(name),
            namespace: Some(CONFIG.KUBE_NAMESPACE.to_string()),
            labels: Some(resources::labels(&body.agent, COMPONENT)),
            ..Default::default()
        },
        spec: Some(PersistentVolumeClaimSpec {
            // An agent's memory is mounted by exactly one runtime pod at a time,
            // so ReadWriteOnce is both sufficient and the most widely supported
            // access mode across storage backends.
            access_modes: Some(vec!["ReadWriteOnce".to_owned()]),
            storage_class_name: Some(storage_class),
            resources: Some(VolumeResourceRequirements {
                requests: Some(BTreeMap::from([(
                    "storage".to_owned(),
                    Quantity(size),
                )])),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let created = resources::volume_api(kube)
        .create(&PostParams::default(), &claim)
        .await
        .map_err(|err| resources::kube_problem(err, "memory volume"))?;

    Ok((StatusCode::CREATED, Json(created.into())))
}
