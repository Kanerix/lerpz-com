use lerpz_axum::error::{HandlerErrorSchema, HandlerResult};
use uuid::Uuid;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    operation_id = "update_group",
    tag = GROUPS_TAG,
    summary = "Update a specific group",
    params(
        ("id" = Uuid, Path, description = "Group ID"),
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
            description = "Group not found",
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