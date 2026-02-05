use lerpz_axum::error::HandlerResult;

use crate::oapi::MODELS_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = MODELS_TAG,
    summary = "Delete a model"
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
