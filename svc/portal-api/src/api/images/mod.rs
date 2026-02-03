use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod edit;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, delete::handler))
        .routes(routes!(edit::handler))
        .with_state(state)
}
