use crate::state::AppState;

use axum::{Router, routing::get};

mod chats;
mod groups;
mod health;
mod images;
mod orgs;
mod usage;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/orgs", orgs::router(state.clone()))
        .nest("/groups", groups::router(state.clone()))
        .nest("/chats", chats::router(state.clone()))
        .nest("/images", images::router(state.clone()))
        .nest("/usage", usage::router(state.clone()))
        .route("/health", get(health::handler))
        .with_state(state)
}
