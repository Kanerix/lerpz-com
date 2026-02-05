use lerpz_axum::error::HandlerResult;

use crate::oapi::HEALTH_TAG;

#[utoipa::path(
    method(get),
    path = "/failure",
    tag = HEALTH_TAG,
    summary = "Force an API failure",
    description = "This is only inlcuded in debug builds",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    "test".parse::<u32>()?;
    Ok(())
}
