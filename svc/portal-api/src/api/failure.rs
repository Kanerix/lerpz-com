use lerpz_axum::error::{HandlerErrorSchema, HandlerResult};

use crate::oapi::HEALTH_TAG;

#[utoipa::path(
    method(get),
    path = "/failure",
    operation_id = "trigger_failure",
    tag = HEALTH_TAG,
    summary = "Force an API failure",
    description = "This is only inlcuded in debug builds",
    responses(
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
    "test".parse::<u32>()?;
    Ok(())
}
