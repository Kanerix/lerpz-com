use lerpz_axum::error::HandlerResult;

use crate::oapi::MODELS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    tag = MODELS_TAG,
    summary = "Update a specific model"
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
