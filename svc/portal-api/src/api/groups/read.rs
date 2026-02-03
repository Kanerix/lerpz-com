use lerpz_axum::error::HandlerResult;

use crate::oapi::CHATS_TAG;

#[utoipa::path(
    method(get),
    path = "/{id}",
    tag = CHATS_TAG,
    summary = "Get a specific group",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
