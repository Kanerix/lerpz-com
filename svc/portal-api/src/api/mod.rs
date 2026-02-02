use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod chats;
mod groups;
mod health;
mod images;
mod orgs;
mod usage;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/orgs", orgs::router(state.clone()))
        .nest("/groups", groups::router(state.clone()))
        .nest("/chats", chats::router(state.clone()))
        .nest("/images", images::router(state.clone()))
        .nest("/usage", usage::router(state.clone()))
        .routes(routes!(health::handler))
        .with_state(state)
}
