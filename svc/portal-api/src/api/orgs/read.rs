use lerpz_axum::problem::{HandlerResult, ProblemSchema};
use uuid::Uuid;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(get),
    path = "/{id}",
    operation_id = "get_org",
    tag = ORGS_TAG,
    summary = "Get a specific organization",
    params(
        ("id" = Uuid, Path, description = "Organization ID"),
    ),
    responses(
        (
            status = OK,
            description = "Not yet implemented"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Resource not found",
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
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
