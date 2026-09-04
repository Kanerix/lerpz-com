use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod job_store;
mod list;
mod status;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(status::handler))
}
