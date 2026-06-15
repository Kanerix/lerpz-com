use lerpz_axum::problem::{HandlerResult, ProblemSchema};

use crate::oapi::SESSIONS_TAG;

#[utoipa::path(
    method(post),
    path = "/{id}/stop",
    operation_id = "stop_session",
    tag = SESSIONS_TAG,
    summary = "Stop a specific session",
    params(
        ("id" = Uuid, Path, description = "Session ID"),
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
