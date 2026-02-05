use lerpz_axum::error::HandlerResult;

use crate::oapi::HEALTH_TAG;

#[utoipa::path(
    method(get),
    path = "/health",
    tag = HEALTH_TAG,
    summary = "Get API health status",
    description = "Return nothing with a status of 200 if everything is okay.",
    responses(
        (status = OK, description = "Success", content_type = "text/plain")
    )
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
