use lerpz_axum::problem::{ProblemSchema, HandlerResult};
use uuid::Uuid;

use crate::oapi::IMAGES_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = IMAGES_TAG,
    operation_id = "delete_image",
    summary = "Delete a specific image",
    params(
        ("id" = Uuid, Path, description = "Image ID"),
    ),
    responses(
        (
            status = OK,
            description = "Image deleted successfully"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Image not found",
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
