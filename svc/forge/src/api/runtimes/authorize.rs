use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use kube::api::ListParams;
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use uuid::Uuid;

use crate::{
    networking,
    oapi::RUNTIMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(get),
    path = "/{runtime_id}/authorize",
    operation_id = "authorize_runtime",
    tag = RUNTIMES_TAG,
    summary = "Authorise access to a runtime",
    description = "Used by Traefik ForwardAuth with the original Authorization header. \
        Requires exactly one non-deleting Forge-managed runtime with a valid agent label \
        and user ownership matching the caller's object and tenant IDs. \
        Forwarded user and owner headers do not grant access. \
        Inaccessible runtimes return 404. Success returns no runtime metadata.",
    params(
        ("runtime_id" = Uuid, Path, description = "Runtime UUID"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "The caller may access the runtime"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token, or missing or empty oid or tid claims",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "No unique runtime accessible to the caller",
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
    Path(runtime_id): Path<Uuid>,
    State(kube): State<KubeClient>,
) -> HandlerResult<StatusCode> {
    let (object_id, tenant_id) = resources::caller_identity(&token)?;
    let runtime_id = runtime_id.to_string();
    let selector = format!(
        "{},{}={runtime_id}",
        resources::owned_selector(None, &token)?,
        networking::RUNTIME_ID_LABEL,
    );

    let deployments = resources::runtime_api(kube)
        .list(&ListParams::default().labels(&selector).limit(2))
        .await
        .map_err(|err| resources::kube_problem(err, "agent runtime"))?;

    let [deployment] = deployments.items.as_slice() else {
        return Err(resources::not_found("agent runtime", &runtime_id));
    };
    let labels = deployment.metadata.labels.as_ref();
    let Some(agent) = labels.and_then(|labels| labels.get(resources::AGENT_LABEL)) else {
        return Err(resources::not_found("agent runtime", &runtime_id));
    };

    if deployment.metadata.deletion_timestamp.is_some()
        || resources::validate_agent(agent).is_err()
        || !resources::is_owned_by(labels, agent, object_id, tenant_id)
    {
        return Err(resources::not_found("agent runtime", &runtime_id));
    }

    Ok(StatusCode::NO_CONTENT)
}
