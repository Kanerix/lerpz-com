use axum::{
    Json,
    extract::{Query, State},
};
use kube::api::ListParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    api::volumes::MemoryVolumeResponse,
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListVolumesQuery {
    /// Only return the caller's volume for this agent
    agent: Option<String>,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_volumes",
    tag = VOLUMES_TAG,
    summary = "List memory volumes",
    description = "Lists only Forge-managed memory volumes with owner type `user` \
        whose owner ID and tenant ID match the caller. Resources without ownership \
        labels or with unsupported owner types are omitted. Creator labels do not grant access.",
    params(ListVolumesQuery),
    responses(
        (
            status = OK,
            description = "The caller's managed memory volumes",
            body = Vec<MemoryVolumeResponse>
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token, or missing or empty oid or tid claims",
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
    Query(query): Query<ListVolumesQuery>,
    State(kube): State<KubeClient>,
) -> HandlerResult<Json<Vec<MemoryVolumeResponse>>> {
    let selector = resources::owned_selector(query.agent.as_deref(), &token)?;

    let claims = resources::volume_api(kube)
        .list(&ListParams::default().labels(&selector))
        .await
        .map_err(|err| resources::kube_problem(err, "memory volumes"))?;

    let volumes = claims
        .items
        .into_iter()
        .map(MemoryVolumeResponse::from)
        .collect();

    Ok(Json(volumes))
}
