use std::time::Instant;

use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, ProblemSchema},
};

use super::AgentMemoryResponse;
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_agent_memory",
    tag = AGENTS_TAG,
    summary = "List agent memory",
    description = "Lists the caller's agent memory, including memory retained after an agent \
        is deleted. Each item reports its availability and whether it is in use.",
    responses(
        (
            status = OK,
            description = "The caller's agent memory",
            body = Vec<AgentMemoryResponse>
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid agent memory request",
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
            description = "Required agent memory information is unavailable or inaccessible",
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
    name = "agent_memory.list",
    skip_all,
    fields(user_id = claims.oid.as_deref(), tenant_id = %claims.tid)
)]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    State(forge): State<ForgeClient>,
) -> HandlerResult<Json<Vec<AgentMemoryResponse>>> {
    let started = Instant::now();

    tracing::debug!("listing agent memory");
    let (volumes, runtimes) =
        tokio::try_join!(forge.list_volumes(&token), forge.list_runtimes(&token))?;

    let memory = volumes
        .into_iter()
        .map(|volume| {
            let in_use = runtimes
                .iter()
                .any(|runtime| runtime.agent == volume.agent && runtime.memory_volume.is_some());
            AgentMemoryResponse::from_volume(volume, in_use)
        })
        .collect::<Vec<_>>();
    tracing::debug!(
        count = memory.len(),
        status = 200,
        elapsed_ms = started.elapsed().as_millis(),
        "agent memory list succeeds"
    );

    Ok(Json(memory))
}
