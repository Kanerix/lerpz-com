use lerpz_axum::error::HandlerResult;

#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
    Ok(())
}
