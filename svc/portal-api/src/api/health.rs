use lerpz_axum::error::HandlerResult;

#[utoipa::path(
    method(get, head),
    path = "/health",
    responses(
        (status = OK, description = "Success", content_type = "text/plain")
    )
)]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
