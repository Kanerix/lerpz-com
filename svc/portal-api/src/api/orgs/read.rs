use lerpz_axum::error::HandlerResult;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(get),
    path = "/{id}",
    tag = ORGS_TAG,
    summary = "Get a specific of organization"
)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
