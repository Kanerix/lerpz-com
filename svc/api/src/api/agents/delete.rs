use std::time::Instant;

use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, ProblemSchema},
};

use super::validate_name;
use crate::{forge::ForgeClient, oapi::AGENTS_TAG, state::AppState};

#[utoipa::path(
    method(delete),
    path = "/{name}",
    operation_id = "delete_agent",
    tag = AGENTS_TAG,
    summary = "Delete a private agent",
    description = "Requests removal of an agent owned by the authenticated user. Its persistent \
        memory is kept and can be reused by a new agent with the same name. Removal may take a moment.",
    params(("name" = String, Path, description = "Agent name")),
    responses(
        (status = NO_CONTENT, description = "Agent removal requested; memory is kept"),
        (status = BAD_REQUEST, description = "Invalid agent name", body = ProblemSchema, content_type = "application/problem+json"),
        (status = UNAUTHORIZED, description = "Missing or invalid authentication", body = ProblemSchema, content_type = "application/problem+json"),
        (status = FORBIDDEN, description = "Agent access denied", body = ProblemSchema, content_type = "application/problem+json"),
        (status = NOT_FOUND, description = "Agent not found or inaccessible", body = ProblemSchema, content_type = "application/problem+json"),
        (status = CONFLICT, description = "The agent changed during removal", body = ProblemSchema, content_type = "application/problem+json"),
        (status = BAD_GATEWAY, description = "Agent removal could not be confirmed", body = ProblemSchema, content_type = "application/problem+json"),
        (status = GATEWAY_TIMEOUT, description = "Agent removal could not be confirmed in time", body = ProblemSchema, content_type = "application/problem+json"),
    ),
)]
#[axum::debug_handler(state = AppState)]
#[tracing::instrument(
    name = "agents.delete",
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
    tracing::info!("deleting agent and preserving memory");
    forge.delete_runtime(&token, &name).await?;
    tracing::info!(
        status = 204,
        memory_kept = true,
        elapsed_ms = started.elapsed().as_millis(),
        "agent removal request succeeds"
    );
    Ok(StatusCode::NO_CONTENT)
}
