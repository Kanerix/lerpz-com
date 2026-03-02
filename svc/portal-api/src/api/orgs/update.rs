use lerpz_axum::error::HandlerResult;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(patch),
    path = "/{id}",
    tag = ORGS_TAG,
    summary = "Update a specific organization"
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
