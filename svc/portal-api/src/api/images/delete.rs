use lerpz_axum::error::HandlerResult;

use crate::oapi::IMAGES_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = IMAGES_TAG,
    summary = "Delete a new image",
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
