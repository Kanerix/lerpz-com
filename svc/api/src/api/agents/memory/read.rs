use std::time::Instant;

use axum::{
    Json,
    extract::{Path, State},
};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, ProblemSchema},
};

use super::{AgentMemoryResponse, not_found, validate_name};
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(get),
    path = "/{name}",
    operation_id = "read_agent_memory",
    tag = AGENTS_TAG,
    summary = "Get agent memory",
    description = "Returns the caller's memory for the named agent, including after the agent \
        is deleted. Reports its availability and whether it is in use. Missing or inaccessible \
        memory returns 404.",
    params(("name" = String, Path, description = "Name of the agent the memory belongs to")),
    responses(
        (
            status = OK,
            description = "The agent's memory",
            body = AgentMemoryResponse
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent name",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid token, or missing caller identity",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = FORBIDDEN,
            description = "Missing required delegated permission, or access was refused",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "The agent memory does not exist or is inaccessible",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = CONFLICT,
            description = "The agent or its memory changed during the request",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = BAD_GATEWAY,
            description = "The agent memory request failed",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = GATEWAY_TIMEOUT,
            description = "The agent memory request timed out",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
#[tracing::instrument(
    name = "agent_memory.read",
    skip_all,
    fields(agent_name, user_id = claims.oid.as_deref(), tenant_id = %claims.tid)
)]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    Path(name): Path<String>,
    State(forge): State<ForgeClient>,
) -> HandlerResult<Json<AgentMemoryResponse>> {
    let started = Instant::now();

    validate_name(&name)?;
    tracing::debug!("reading agent memory");
    let volume = forge
        .read_volume(&token, &name)
        .await?
        .ok_or_else(not_found)?;
    let in_use = forge
        .read_runtime(&token, &name)
        .await?
        .is_some_and(|runtime| runtime.agent == volume.agent && runtime.memory_volume.is_some());
    let response = AgentMemoryResponse::from_volume(volume, in_use);
    tracing::debug!(status = 200, memory_status = ?response.status, in_use, elapsed_ms = started.elapsed().as_millis(), "agent memory read succeeds");
    Ok(Json(response))
}
