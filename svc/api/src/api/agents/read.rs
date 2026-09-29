use std::time::Instant;

use axum::{
    Json,
    extract::{Path, State},
};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, ProblemSchema},
};

use super::{AgentResponse, not_found, validate_name};
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(get),
    path = "/{name}",
    operation_id = "read_agent",
    tag = AGENTS_TAG,
    summary = "Read a private agent",
    description = "Returns an agent owned by the authenticated user. Missing and inaccessible agents return 404.",
    params(("name" = String, Path, description = "Agent name")),
    responses(
        (status = OK, description = "The agent", body = AgentResponse),
        (status = BAD_REQUEST, description = "Invalid agent name", body = ProblemSchema, content_type = "application/problem+json"),
        (status = UNAUTHORIZED, description = "Missing or invalid authentication", body = ProblemSchema, content_type = "application/problem+json"),
        (status = FORBIDDEN, description = "Agent access denied", body = ProblemSchema, content_type = "application/problem+json"),
        (status = NOT_FOUND, description = "Agent not found or inaccessible", body = ProblemSchema, content_type = "application/problem+json"),
        (status = BAD_GATEWAY, description = "The agent could not be read", body = ProblemSchema, content_type = "application/problem+json"),
        (status = GATEWAY_TIMEOUT, description = "The agent could not be read in time", body = ProblemSchema, content_type = "application/problem+json"),
    ),
)]
#[axum::debug_handler(state = AppState)]
#[tracing::instrument(name = "agents.read", skip_all, fields(agent_name, user_id = claims.oid.as_deref(), tenant_id = %claims.tid))]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    Path(name): Path<String>,
    State(forge): State<ForgeClient>,
) -> HandlerResult<Json<AgentResponse>> {
    let started = Instant::now();

    validate_name(&name)?;
    tracing::debug!("reading agent");
    let runtime = forge
        .read_runtime(&token, &name)
        .await?
        .ok_or_else(not_found)?;
    let response = AgentResponse::from_runtime(runtime);
    tracing::debug!(status = 200, agent_status = ?response.status, elapsed_ms = started.elapsed().as_millis(), "agent read succeeds");
    Ok(Json(response))
}
