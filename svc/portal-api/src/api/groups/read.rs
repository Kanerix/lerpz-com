use lerpz_axum::error::HandlerResult;

use crate::oapi::GROUPS_TAG;

#[utoipa::path(
    method(get),
    path = "/{id}",
    tag = GROUPS_TAG,
    summary = "Get a specific group",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
