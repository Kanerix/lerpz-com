use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod analysis;
mod create;
mod list;
mod status;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(status::handler))
        .routes(routes!(analysis::handler))
}
