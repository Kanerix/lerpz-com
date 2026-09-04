use axum::{
    Json,
    extract::{Path, State},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    api::runtimes::AgentRuntime,
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(get),
    path = "/{agent}",
    operation_id = "read_runtime",
    tag = RUNTIMES_TAG,
    summary = "Get an agent's runtime",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = OK,
            description = "The agent's runtime",
            body = AgentRuntime
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
            description = "The agent has no runtime",
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
) -> HandlerResult<Json<AgentRuntime>> {
    resources::validate_agent(&agent)?;

    let name = resources::runtime_name(&agent);

    let deployment = resources::runtime_api(kube)
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?
        .ok_or_else(|| resources::not_found("agent runtime", &name))?;

    if resources::agent_of(deployment.metadata.labels.as_ref()).as_deref() != Some(agent.as_str()) {
        return Err(resources::not_found("agent runtime", &name));
    }

    Ok(Json(deployment.into()))
}
