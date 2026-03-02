use lerpz_axum::error::HandlerResult;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(delete),
    path = "/{id}",
    tag = ORGS_TAG,
    summary = "Delete a new organization",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
