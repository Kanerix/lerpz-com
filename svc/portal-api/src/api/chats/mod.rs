use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod list;
mod message;
mod read;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler))
        .routes(routes!(create::handler))
        .routes(routes!(read::handler))
        .routes(routes!(message::handler))
}