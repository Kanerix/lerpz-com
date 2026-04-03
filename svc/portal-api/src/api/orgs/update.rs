use lerpz_axum::error::{HandlerErrorSchema, HandlerResult};
use uuid::Uuid;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    tag = ORGS_TAG,
    summary = "Update a specific organization",
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
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Resource not found",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = HandlerErrorSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}