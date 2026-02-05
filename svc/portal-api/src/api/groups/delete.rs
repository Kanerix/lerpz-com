use lerpz_axum::error::HandlerResult;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = GROUPS_TAG,
    summary = "Delete a group"
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
