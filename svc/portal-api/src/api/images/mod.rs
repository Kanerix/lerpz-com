use crate::state::AppState;

use axum::{Router, routing::post};

mod create;
mod edit;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(create::handler))
        .route("/edit", post(edit::handler))
        .with_state(state)
}
