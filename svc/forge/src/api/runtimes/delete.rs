use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use kube::api::DeleteParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(delete),
    path = "/{agent}",
    operation_id = "delete_runtime",
    tag = RUNTIMES_TAG,
    summary = "Tear down an agent's runtime",
    description = "Deletes the agent's `Deployment` and the pods it owns. The \
        agent's memory volume is left intact so the runtime can be re-created \
        against the same memory.",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "The runtime was torn down"
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
) -> HandlerResult<StatusCode> {
    resources::validate_agent(&agent)?;

    let name = resources::runtime_name(&agent);
    let api = resources::runtime_api(kube);

    // Read before deleting so an unlabelled deployment that happens to match the
    // derived name is never removed by Forge.
    let deployment = api
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?
        .ok_or_else(|| resources::not_found("agent runtime", &name))?;

    if resources::agent_of(deployment.metadata.labels.as_ref()).as_deref() != Some(agent.as_str()) {
        return Err(resources::not_found("agent runtime", &name));
    }

    tracing::info!(%agent, %name, "tearing down agent runtime");

    api.delete(&name, &DeleteParams::default())
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?;

    Ok(StatusCode::NO_CONTENT)
}
