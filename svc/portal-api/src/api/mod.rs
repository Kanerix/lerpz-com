use crate::state::AppState;

use axum::{Router, routing::get};

mod chats;
mod health;
mod images;
mod org;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/org", org::router(state.clone()))
        .nest("/chat", chats::router(state.clone()))
        .nest("/image", images::router(state.clone()))
        .route("/health", get(health::handler))
        .with_state(state)
}
