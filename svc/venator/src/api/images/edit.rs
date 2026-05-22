use lerpz_axum::problem::{HandlerResult, ProblemSchema};

use crate::oapi::IMAGES_TAG;

#[utoipa::path(
    method(post),
    path = "/edit",
    operation_id = "edit_image",
    tag = IMAGES_TAG,
    summary = "Create a new image from existing images",
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
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
