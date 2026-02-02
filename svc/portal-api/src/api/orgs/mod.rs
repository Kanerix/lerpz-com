use crate::state::AppState;

use axum::routing::get;
use utoipa_axum::router::OpenApiRouter;

mod create;
mod delete;
mod list;
mod read;
mod update;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route("/", get(list::handler).post(create::handler))
        .route(
            "/{id}",
            get(read::handler)
                .put(update::handler)
                .delete(delete::handler),
        )
        .with_state(state)
}
