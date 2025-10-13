use crate::state::AppState;

use axum::Router;

mod dept;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .nest("/dept", dept::router(state.clone()))
        .with_state(state)
}
