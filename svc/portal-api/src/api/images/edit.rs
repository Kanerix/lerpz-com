use lerpz_axum::error::HandlerResult;

use crate::oapi::IMAGES_TAG;

#[utoipa::path(
    method(post),
    path = "/edit",
    tag = IMAGES_TAG,
    summary = "Create a new image from existing images",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
