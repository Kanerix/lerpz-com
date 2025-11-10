use crate::state::AppState;

use axum::{Router, routing::post};

mod create;
mod list;
mod read;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::handler).get(list::handler))
        .route("/{id}", post(read::handler))
        .with_state(state)
}
