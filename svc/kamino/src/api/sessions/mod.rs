use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod delete;
mod start;
mod stop;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(start::handler))
        .routes(routes!(stop::handler))
        .routes(routes!(delete::handler))
}
