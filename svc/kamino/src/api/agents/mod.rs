use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod update;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler))
        .routes(routes!(update::handler, delete::handler))
}
