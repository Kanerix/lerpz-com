use lerpz_axum::error::HandlerResult;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(get),
    path = "/",
    tag = GROUPS_TAG,
    summary = "Get a list of groups",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
