use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod analysis;
mod create;
mod delete;
mod edit;
mod list;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(edit::handler))
        .routes(routes!(delete::handler))
        .routes(routes!(analysis::handler))
}
