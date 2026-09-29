use std::time::Duration;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use kube::api::ListParams;
use lerpz_axum::problem::{HandlerResult, ProblemSchema};
use serde::Serialize;
use tokio::time::timeout;
use utoipa::ToSchema;

use crate::oapi::HEALTH_TAG;
use crate::resources;
use crate::state::{AppState, KubeClient};

/// Maximum time to wait for the cluster health ping before considering the
/// Kubernetes API unreachable.
const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheckResponse {
    cluster: bool,
}

#[utoipa::path(
    method(get),
    path = "/health",
    operation_id = "health_check",
    tag = HEALTH_TAG,
    summary = "Get Forge health status",
    description = "Verifies connectivity to the Kubernetes API, always returns 200 if health check succeeds.",
    security(()),
    responses(
        (
            status = OK,
            description = "Service is healthy",
            body = HealthCheckResponse
        ),
        (
            status = SERVICE_UNAVAILABLE,
            description = "The Kubernetes API is unreachable",
            body = HealthCheckResponse
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
    State(kube): State<KubeClient>,
) -> HandlerResult<(StatusCode, Json<HealthCheckResponse>)> {
    let params = ListParams::default()
        .labels(&resources::managed_selector())
        .limit(1);

    let cluster_ok = timeout(
        HEALTH_CHECK_TIMEOUT,
        resources::volume_api(kube).list_metadata(&params),
    )
    .await
    .is_ok_and(|result| result.is_ok());

    let status_code = if cluster_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((
        status_code,
        Json(HealthCheckResponse {
            cluster: cluster_ok,
        }),
    ))
}
