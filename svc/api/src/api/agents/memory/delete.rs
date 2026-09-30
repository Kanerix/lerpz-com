use std::time::Instant;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, Problem, ProblemSchema},
};

use super::{not_found, validate_name};
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(delete),
    path = "/{name}",
    operation_id = "delete_agent_memory",
    tag = AGENTS_TAG,
    summary = "Remove agent memory",
    description = "Requests removal of the caller's memory for the named agent. Memory mounted \
        by the matching agent cannot be removed. Missing or inaccessible memory returns 404.",
    params(("name" = String, Path, description = "Name of the agent the memory belongs to")),
    responses(
        (
            status = NO_CONTENT,
            description = "Agent memory removal was requested"
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
            description = "The memory is in use or changed during the request",
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
    name = "agent_memory.delete",
    skip_all,
    fields(agent_name, user_id = claims.oid.as_deref(), tenant_id = %claims.tid)
)]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    Path(name): Path<String>,
    State(forge): State<ForgeClient>,
) -> HandlerResult<StatusCode> {
    let started = Instant::now();

    validate_name(&name)?;

    tracing::info!("checking agent memory before removal");
    let volume = forge
        .read_volume(&token, &name)
        .await?
        .ok_or_else(not_found)?;

    let in_use = forge
        .read_runtime(&token, &name)
        .await?
        .is_some_and(|runtime| runtime.agent == volume.agent && runtime.memory_volume.is_some());
    if in_use {
        tracing::warn!(
            status = 409,
            reason = "memory_in_use",
            "rejecting agent memory removal"
        );
        return Err(Problem::new(
            StatusCode::CONFLICT,
            "Agent memory in use",
            "Delete the agent before requesting removal of its memory.",
        ));
    }

    forge.delete_volume(&token, &name).await?;
    tracing::info!(
        status = 204,
        elapsed_ms = started.elapsed().as_millis(),
        "agent memory removal request succeeds"
    );

    Ok(StatusCode::NO_CONTENT)
}
