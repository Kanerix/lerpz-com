use crate::state::AppState;

use axum::{Router, routing::get};

mod chat;
mod dept;
mod health;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/dept", dept::router(state.clone()))
        .route("/chat", get(chat::handler))
        .route("/health", get(health::handler))
        .with_state(state)
}
