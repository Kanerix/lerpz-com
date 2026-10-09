use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod edit;
mod list;
mod message;
mod message_delete;
mod read;
mod stream;
mod update;

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler))
        .routes(routes!(create::handler))
        .routes(routes!(read::handler))
        .routes(routes!(message::handler))
        .routes(routes!(message_delete::handler))
        .routes(routes!(edit::handler))
        .routes(routes!(update::handler))
        .routes(routes!(delete::handler))
}
