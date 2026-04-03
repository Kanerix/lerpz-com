use lerpz_axum::error::{HandlerErrorSchema, HandlerResult};

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(post),
    path = "/",
    tag = GROUPS_TAG,
    summary = "Create a new group",
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
