use lerpz_axum::error::HandlerResult;

use crate::oapi::CHATS_TAG;

#[utoipa::path(
    method(get),
    path = "/",
    tag = CHATS_TAG,
    summary = "Get a list of chats",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
