use crate::state::AppState;

use axum::{Router, routing::get};

mod create;
mod delete;
mod list;
mod read;
mod update;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list::handler).post(create::handler))
        .route(
            "/{id}",
            get(read::handler)
                .put(update::handler)
                .delete(delete::handler),
        )
        .with_state(state)
}
