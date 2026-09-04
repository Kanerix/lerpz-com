use axum::{Json, extract::{Query, State}};
use kube::api::ListParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    api::volumes::MemoryVolume,
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListVolumesQuery {
    /// Only return the volume belonging to this agent
    agent: Option<String>,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_volumes",
    tag = VOLUMES_TAG,
    summary = "List memory volumes",
    description = "Lists the memory volumes Forge manages in its namespace. \
        Claims created by anything other than Forge are never returned.",
    params(ListVolumesQuery),
    responses(
        (
            status = OK,
            description = "The managed memory volumes",
            body = Vec<MemoryVolume>
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
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
    Query(query): Query<ListVolumesQuery>,
) -> HandlerResult<Json<Vec<MemoryVolume>>> {
    let selector = match query.agent.as_deref() {
        Some(agent) => {
            resources::validate_agent(agent)?;
            resources::agent_selector(agent)
        }
        None => resources::managed_selector(),
    };

    let claims = resources::volume_api(kube)
        .list(&ListParams::default().labels(&selector))
        .await
        .map_err(|err| resources::kube_problem(err, "memory volumes"))?;

    let volumes = claims.items.into_iter().map(MemoryVolume::from).collect();

    Ok(Json(volumes))
}
