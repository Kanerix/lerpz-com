use crate::state::AppState;

use axum::{Router, routing::get};

mod chat;
mod org;
mod health;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/org", org::router(state.clone()))
        .route("/chat", get(chat::handler))
        .route("/health", get(health::handler))
        .with_state(state)
}
