use crate::state::AppState;

use axum::routing::post;
use utoipa_axum::router::OpenApiRouter;

mod create;
mod list;
mod read;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route("/", post(create::handler).get(list::handler))
        .route("/{id}", post(read::handler))
        .with_state(state)
}
