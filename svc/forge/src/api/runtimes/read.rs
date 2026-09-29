use axum::{
    Json,
    extract::{Path, State},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    api::runtimes::AgentRuntimeResponse,
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
    description = "Returns the agent's runtime only when it is managed by Forge, \
        has owner type `user`, and its owner ID and tenant ID match the caller. \
        Inaccessible resources, including those without ownership labels, return 404. \
        Creator labels do not grant access.",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = OK,
            description = "The agent's runtime",
            body = AgentRuntimeResponse
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent identifier",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token, or missing or empty oid or tid claims",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "The agent has no runtime accessible to the caller",
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
    Path(agent): Path<String>,
    State(kube): State<KubeClient>,
) -> HandlerResult<Json<AgentRuntimeResponse>> {
    let (object_id, tenant_id) = resources::caller_identity(&token)?;
    resources::validate_agent(&agent)?;

    let name = resources::runtime_name(&agent);

    let deployment = resources::runtime_api(kube)
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?
        .ok_or_else(|| resources::not_found("agent runtime", &name))?;

    if !resources::is_owned_by(
        deployment.metadata.labels.as_ref(),
        &agent,
        object_id,
        tenant_id,
    ) {
        return Err(resources::not_found("agent runtime", &name));
    }

    Ok(Json(deployment.into()))
}
