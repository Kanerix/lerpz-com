use lerpz_axum::error::HandlerResult;

use crate::oapi::ORGS_TAG;

#[utoipa::path(
    method(post),
    path = "/",
    tag = ORGS_TAG,
    summary = "Create a new organization"
)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
