use crate::state::AppState;

use axum::{
    Router,
    routing::{get, post},
};

mod create;
mod delete;
mod list;
mod read;
mod update;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(create::handler).get(list::handler))
        .route(
            "/{id}",
            get(read::handler)
                .put(update::handler)
                .delete(delete::handler),
        )
        .with_state(state)
}
