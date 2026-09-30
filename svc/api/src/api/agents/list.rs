use std::time::Instant;

use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, ProblemSchema},
};

use super::AgentResponse;
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_agents",
    tag = AGENTS_TAG,
    summary = "List private agents",
    description = "Lists only agents owned by the authenticated user. Retained memory is listed separately.",
    responses(
        (
            status = OK,
            description = "The user's agents",
            body = Vec<AgentResponse>
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = FORBIDDEN,
            description = "Agent access denied",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = BAD_GATEWAY,
            description = "Agents could not be read",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = GATEWAY_TIMEOUT,
            description = "Agents could not be read in time",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
#[tracing::instrument(
    name = "agents.list",
    skip_all,
    fields(user_id = claims.oid.as_deref(), tenant_id = %claims.tid)
)]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    State(forge): State<ForgeClient>,
) -> HandlerResult<Json<Vec<AgentResponse>>> {
    let started = Instant::now();

    tracing::debug!("listing agents");
    let runtimes = forge.list_runtimes(&token).await?;
    let agents = runtimes
        .into_iter()
        .map(AgentResponse::from_runtime)
        .collect::<Vec<_>>();
    tracing::debug!(
        count = agents.len(),
        status = 200,
        elapsed_ms = started.elapsed().as_millis(),
        "agent list succeeds"
    );
    Ok(Json(agents))
}
