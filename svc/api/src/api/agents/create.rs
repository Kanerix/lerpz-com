use std::time::Instant;

use axum::{Json, extract::State, http::StatusCode};
use lerpz_axum::{
    middleware::azure::{AzureAccessToken, RawAzureToken},
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Deserialize;
use utoipa::ToSchema;

use super::{AgentResponse, validate_name};
use crate::{
    forge::{CreateRuntimeRequest, CreateVolumeRequest, ForgeClient},
    oapi::AGENTS_TAG,
    state::AppState,
};

#[derive(Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateAgentRequest {
    /// Agent name. Use 1 to 40 lowercase letters, digits or hyphens, starting
    /// and ending with a letter or digit.
    #[schema(
        min_length = 1,
        max_length = 40,
        pattern = "^[a-z0-9]([a-z0-9-]{0,38}[a-z0-9])?$"
    )]
    name: String,
    /// Create new persistent memory, reuse this name's existing memory, or run without it.
    memory: AgentMemoryChoice,
    /// Optional advanced limits. Omitted limits use platform settings.
    resource_limits: Option<AgentResourceLimitsRequest>,
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AgentMemoryChoice {
    New,
    Existing,
    None,
}

#[derive(Default, Deserialize, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct AgentResourceLimitsRequest {
    /// CPU limit in millicores. 1000 millicores is one CPU core.
    #[schema(minimum = 1, example = 500)]
    cpu_millicores: Option<u32>,
    /// RAM limit in mebibytes. This is separate from persistent agent memory.
    #[schema(minimum = 1, example = 512)]
    memory_mib: Option<u32>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_agent",
    tag = AGENTS_TAG,
    summary = "Create a private agent",
    description = "Creates an agent and prepares its selected memory. Existing memory must belong \
        to the caller and have the same agent name. New memory uses platform storage settings. \
        A successful response does not mean the agent is ready yet. Creation is not retried \
        automatically. If startup fails, memory is kept; check the agent and memory before retrying.",
    request_body(content = CreateAgentRequest, content_type = "application/json"),
    responses(
        (
            status = CREATED,
            description = "The agent was created",
            body = AgentResponse
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid name or resource limits",
            body = ProblemSchema,
            content_type = "application/problem+json"
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
            status = NOT_FOUND,
            description = "Existing memory was not found or is inaccessible",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = CONFLICT,
            description = "The name or memory is already in use or has changed",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNPROCESSABLE_ENTITY,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = BAD_GATEWAY,
            description = "Agent creation could not be completed",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = GATEWAY_TIMEOUT,
            description = "Agent creation could not be confirmed in time",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
#[tracing::instrument(
    name = "agents.create",
    skip_all,
    fields(agent_name, user_id = claims.oid.as_deref(), tenant_id = %claims.tid, memory_mode)
)]
pub async fn handler(
    claims: AzureAccessToken,
    RawAzureToken(token): RawAzureToken,
    State(forge): State<ForgeClient>,
    Json(body): Json<CreateAgentRequest>,
) -> HandlerResult<(StatusCode, Json<AgentResponse>)> {
    let started = Instant::now();

    validate_name(&body.name)?;
    let memory_mode = match body.memory {
        AgentMemoryChoice::New => "new",
        AgentMemoryChoice::Existing => "existing",
        AgentMemoryChoice::None => "none",
    };
    tracing::Span::current().record("memory_mode", memory_mode);
    let limits = body.resource_limits.unwrap_or_default();
    if limits.cpu_millicores == Some(0) || limits.memory_mib == Some(0) {
        tracing::warn!(
            status = 400,
            reason = "invalid_resource_limits",
            "rejecting agent creation"
        );
        return Err(Problem::new(
            StatusCode::BAD_REQUEST,
            "Invalid resource limits",
            "CPU and RAM limits must be greater than zero when supplied.",
        ));
    }

    tracing::info!("creating agent");
    tracing::debug!(
        cpu_millicores = limits.cpu_millicores,
        memory_mib = limits.memory_mib,
        "applying agent resource limits"
    );

    if forge.read_runtime(&token, &body.name).await?.is_some() {
        tracing::warn!(
            status = 409,
            reason = "agent_exists",
            "rejecting agent creation"
        );
        return Err(Problem::new(
            StatusCode::CONFLICT,
            "Agent already exists",
            "An agent with this name already exists.",
        ));
    }

    let new_memory = matches!(body.memory, AgentMemoryChoice::New);
    let runtime_request = CreateRuntimeRequest {
        agent: body.name.clone(),
        replicas: 1,
        mount_memory: !matches!(body.memory, AgentMemoryChoice::None),
        cpu_limit: limits.cpu_millicores.map(|cpu| format!("{cpu}m")),
        memory_limit: limits.memory_mib.map(|memory| format!("{memory}Mi")),
    };

    match body.memory {
        AgentMemoryChoice::New => {
            let memory_request = CreateVolumeRequest {
                agent: body.name.clone(),
            };
            tracing::info!("creating agent memory");
            forge.create_volume(&token, &memory_request).await?;
            tracing::info!("agent memory creation succeeds");
        }
        AgentMemoryChoice::Existing => {
            tracing::debug!("checking existing agent memory");
            let memory = forge
                .read_volume(&token, &body.name)
                .await?
                .ok_or_else(|| {
                    tracing::warn!(
                        status = 404,
                        reason = "memory_not_found",
                        "rejecting agent creation"
                    );
                    Problem::new(
                        StatusCode::NOT_FOUND,
                        "Agent memory not found",
                        "No accessible memory exists for this agent name.",
                    )
                })?;
            if memory.phase.as_deref() == Some("Lost") {
                tracing::warn!(
                    status = 409,
                    reason = "memory_unavailable",
                    "rejecting agent creation"
                );
                return Err(Problem::new(
                    StatusCode::CONFLICT,
                    "Agent memory unavailable",
                    "This agent's memory is unavailable and cannot be reused.",
                ));
            }
        }
        AgentMemoryChoice::None => {}
    }

    tracing::debug!("starting agent");
    let runtime = forge.create_runtime(&token, &runtime_request).await.map_err(|problem| {
        if !new_memory {
            return problem;
        }

        let log_id = uuid::Uuid::new_v4().to_string();
        tracing::error!(
            %log_id,
            status = problem.status().as_u16(),
            memory_kept = true,
            elapsed_ms = started.elapsed().as_millis(),
            "agent creation fails after creating memory"
        );

        Problem::new(
            problem.status(),
            "Agent creation incomplete",
            format!("{} New memory was created and has been kept. Check the agent before retrying with existing memory.", problem.detail()),
        )
        .with_log_id(log_id)
    })?;

    let response = AgentResponse::from_runtime(runtime);
    tracing::info!(status = 201, agent_status = ?response.status, elapsed_ms = started.elapsed().as_millis(), "agent creation succeeds");
    Ok((StatusCode::CREATED, Json(response)))
}
