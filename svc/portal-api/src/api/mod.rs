use crate::state::AppState;

use axum::{Router, routing::get};

mod chat;
mod health;
mod image;
mod org;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/org", org::router(state.clone()))
        .nest("/chat", chat::router(state.clone()))
        .nest("/image", image::router(state.clone()))
        .route("/health", get(health::handler))
        .with_state(state)
}
