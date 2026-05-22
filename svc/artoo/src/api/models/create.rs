use lerpz_axum::problem::{HandlerResult, ProblemSchema};

use crate::oapi::MODELS_TAG;

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_model",
    tag = MODELS_TAG,
    summary = "Create a new model",
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
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
