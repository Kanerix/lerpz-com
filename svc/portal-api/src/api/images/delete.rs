use lerpz_axum::error::HandlerResult;

use crate::oapi::IMAGES_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = IMAGES_TAG,
    summary = "Delete a specific image",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
