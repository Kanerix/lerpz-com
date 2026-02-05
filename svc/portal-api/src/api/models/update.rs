use lerpz_axum::error::HandlerResult;

use crate::oapi::MODELS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    tag = MODELS_TAG,
    summary = "Update a specific model"
)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
