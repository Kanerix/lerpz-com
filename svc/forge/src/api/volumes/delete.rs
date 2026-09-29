use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};

use crate::{
    oapi::VOLUMES_TAG,
    resources,
    state::{AppState, KubeClient},
};

#[utoipa::path(
    method(delete),
    path = "/{agent}",
    operation_id = "delete_volume",
    tag = VOLUMES_TAG,
    summary = "Reclaim an agent's memory volume",
    description = "Deletes the agent's `PersistentVolumeClaim` only when it is managed \
        by Forge, has owner type `user`, and its owner ID and tenant ID match the caller. \
        Inaccessible resources, including those without ownership labels, return 404. \
        Creator labels do not grant access. Whether the \
        underlying data is destroyed depends on the StorageClass reclaim policy. \
        Delete the agent's runtime first. A claim still mounted by a running \
        pod stays `Terminating` until the pod is gone.",
    params(
        ("agent" = String, Path, description = "Identifier of the agent"),
    ),
    responses(
        (
            status = NO_CONTENT,
            description = "The memory volume was reclaimed"
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
            status = FORBIDDEN,
            description = "Missing required delegated user permission, or cluster access was refused",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "The agent has no memory volume accessible to the caller",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = CONFLICT,
            description = "Resource changed while the deletion was being authorised. Retry the request.",
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
) -> HandlerResult<StatusCode> {
    let (object_id, tenant_id) = resources::caller_identity(&token)?;
    resources::validate_agent(&agent)?;

    let name = resources::memory_volume_name(&agent);
    let api = resources::volume_api(kube);

    let claim = api
        .get_opt(&name)
        .await
        .map_err(|err| resources::kube_problem(err, "memory volume"))?
        .ok_or_else(|| resources::not_found("memory volume", &name))?;

    if !resources::is_owned_by(claim.metadata.labels.as_ref(), &agent, object_id, tenant_id) {
        return Err(resources::not_found("memory volume", &name));
    }

    let params = resources::delete_params(&claim.metadata)?;

    tracing::info!(%agent, %name, "reclaiming memory volume");

    api.delete(&name, &params).await.map_err(|err| match &err {
        kube::Error::Api(response) if response.code == 409 => Problem::new(
            StatusCode::CONFLICT,
            "Resource changed",
            "Resource changed while the deletion was being authorised. Retry the request.",
        )
        .with_error(err),
        _ => resources::kube_problem(err, "memory volume"),
    })?;

    Ok(StatusCode::NO_CONTENT)
}
