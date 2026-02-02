use crate::state::AppState;

use axum::routing::post;
use utoipa_axum::router::OpenApiRouter;

mod create;
mod edit;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .route("/create", post(create::handler))
        .route("/edit", post(edit::handler))
        .with_state(state)
}
