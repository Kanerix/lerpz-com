use axum::{extract::{Path, State}, http::StatusCode};
use kube::api::DeleteParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(delete),
    path = "/{agent}",
    operation_id = "delete_volume",
    tag = VOLUMES_TAG,
    summary = "Reclaim an agent's memory volume",
    description = "Deletes the agent's `PersistentVolumeClaim`. Whether the \
        underlying data is destroyed depends on the StorageClass reclaim policy. \
        Delete the agent's runtime first — a claim still mounted by a running \
        pod stays `Terminating` until the pod is gone.",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "The memory volume was reclaimed"
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent identifier",
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
            status = NOT_FOUND,
            description = "The agent has no memory volume",
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
    Path(agent): Path<String>,
) -> HandlerResult<StatusCode> {
    resources::validate_agent(&agent)?;

    let name = resources::memory_volume_name(&agent);
    let api = resources::volume_api(kube);

    // Read before deleting so an unlabelled claim that happens to match the
    // derived name is never removed by Forge.
    let claim = api
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "memory volume"))?
        .ok_or_else(|| resources::not_found("memory volume", &name))?;

    if resources::agent_of(claim.metadata.labels.as_ref()).as_deref() != Some(agent.as_str()) {
        return Err(resources::not_found("memory volume", &name));
    }

    tracing::info!(%agent, %name, "reclaiming memory volume");

    api.delete(&name, &DeleteParams::default())
        .await
        .map_err(|err| resources::kube_problem(err, "memory volume"))?;

    Ok(StatusCode::NO_CONTENT)
}
