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
    api::runtimes::AgentRuntimeResponse,
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListRuntimesQuery {
    /// Only return the caller's runtime for this agent
    agent: Option<String>,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_runtimes",
    tag = RUNTIMES_TAG,
    summary = "List agent runtimes",
    description = "Lists only Forge-managed agent runtimes with owner type `user` \
        whose owner ID and tenant ID match the caller. Resources without ownership \
        labels or with unsupported owner types are omitted. Creator labels do not grant access.",
    params(ListRuntimesQuery),
    responses(
        (
            status = OK,
            description = "The caller's managed agent runtimes",
            body = Vec<AgentRuntimeResponse>
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token, or missing or empty oid or tid claims",
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
    Query(query): Query<ListRuntimesQuery>,
    State(kube): State<KubeClient>,
) -> HandlerResult<Json<Vec<AgentRuntimeResponse>>> {
    let selector = resources::owned_selector(query.agent.as_deref(), &token)?;

    let deployments = resources::runtime_api(kube)
        .list(&ListParams::default().labels(&selector))
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtimes"))?;

    let runtimes = deployments
        .items
        .into_iter()
        .map(AgentRuntimeResponse::from)
        .collect();

    Ok(Json(runtimes))
}
