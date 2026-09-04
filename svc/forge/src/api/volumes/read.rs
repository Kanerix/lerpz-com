use axum::{
    Json,
    extract::{Path, State},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    api::volumes::MemoryVolume,
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(get),
    path = "/{agent}",
    operation_id = "read_volume",
    tag = VOLUMES_TAG,
    summary = "Get an agent's memory volume",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = OK,
            description = "The agent's memory volume",
            body = MemoryVolume
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
) -> HandlerResult<Json<MemoryVolume>> {
    resources::validate_agent(&agent)?;

    let name = resources::memory_volume_name(&agent);

    let claim = resources::volume_api(kube)
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "memory volume"))?
        .ok_or_else(|| resources::not_found("memory volume", &name))?;

    // A claim that exists but is not ours must not be readable through Forge,
    // otherwise the name-mangling scheme becomes a way to inspect unrelated
    // claims in the namespace.
    if resources::agent_of(claim.metadata.labels.as_ref()).as_deref() != Some(agent.as_str()) {
        return Err(resources::not_found("memory volume", &name));
    }

    Ok(Json(claim.into()))
}
