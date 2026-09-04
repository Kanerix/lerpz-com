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
    api::runtimes::AgentRuntime,
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[derive(Debug, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListRuntimesQuery {
    /// Only return the runtime belonging to this agent
    agent: Option<String>,
}

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_runtimes",
    tag = RUNTIMES_TAG,
    summary = "List agent runtimes",
    description = "Lists the agent runtimes Forge manages in its namespace. \
        Deployments created by anything other than Forge are never returned.",
    params(ListRuntimesQuery),
    responses(
        (
            status = OK,
            description = "The managed agent runtimes",
            body = Vec<AgentRuntime>
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
    Query(query): Query<ListRuntimesQuery>,
) -> HandlerResult<Json<Vec<AgentRuntime>>> {
    let selector = match query.agent.as_deref() {
        Some(agent) => {
            resources::validate_agent(agent)?;
            resources::agent_selector(agent)
        }
        None => resources::managed_selector(),
    };

    let deployments = resources::runtime_api(kube)
        .list(&ListParams::default().labels(&selector))
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtimes"))?;

    let runtimes = deployments
        .items
        .into_iter()
        .map(AgentRuntime::from)
        .collect();

    Ok(Json(runtimes))
}
