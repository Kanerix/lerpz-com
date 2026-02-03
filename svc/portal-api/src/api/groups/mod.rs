use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod list;
mod read;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(read::handler))
        .with_state(state)
}
