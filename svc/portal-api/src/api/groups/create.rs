use lerpz_axum::error::HandlerResult;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(post),
    path = "/",
    tag = GROUPS_TAG,
    summary = "Create a new group"
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
