use lerpz_axum::error::HandlerResult;

use crate::oapi::MODELS_TAG;

#[utoipa::path(
    method(post),
    path = "/",
    tag = MODELS_TAG,
    summary = "Create a new model"
)]
#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
