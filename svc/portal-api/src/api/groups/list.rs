use lerpz_axum::problem::{ProblemSchema, HandlerResult};

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(get),
    path = "/",
    operation_id = "list_groups",
    tag = GROUPS_TAG,
    summary = "Get a list of groups",
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
