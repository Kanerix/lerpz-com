use lerpz_axum::error::HandlerResult;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(get),
    path = "/",
    tag = ORGS_TAG,
    summary = "Get a list of organization"
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
