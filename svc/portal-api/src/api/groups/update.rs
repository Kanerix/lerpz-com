use lerpz_axum::error::HandlerResult;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    tag = GROUPS_TAG,
    summary = "Update a specific group"
)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
