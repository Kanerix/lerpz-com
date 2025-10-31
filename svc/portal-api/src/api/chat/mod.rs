use crate::state::AppState;

use axum::{
    Router,
    routing::{post}
};

mod create;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(create::handler))
        .with_state(state)
}
