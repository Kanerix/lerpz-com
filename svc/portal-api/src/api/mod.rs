use crate::state::AppState;

use axum::{
    Router,
    routing::{get, post},
};

mod chat;
mod health;
mod org;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/org", org::router(state.clone()))
        .route("/chat", post(chat::handler))
        .route("/health", get(health::handler))
        .with_state(state)
}
