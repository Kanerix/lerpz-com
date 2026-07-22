use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod agents;
mod chats;
mod enhance;
mod groups;
mod health;
mod images;
mod models;
mod sessions;
mod settings;
mod videos;

#[cfg(debug_assertions)]
mod failure;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    let router = OpenApiRouter::new()
        .nest("/groups", groups::router())
        .nest("/chats", chats::router())
        .nest("/images", images::router())
        .nest("/videos", videos::router())
        .nest("/enhance", enhance::router())
        .nest("/models", models::router())
        .nest("/agents", agents::router())
        .nest("/sessions", sessions::router())
        .nest("/settings", settings::router())
        .routes(routes!(health::handler))
        .with_state(state);

    #[cfg(debug_assertions)]
    let router = OpenApiRouter::new()
        .routes(routes!(failure::handler))
        .merge(router);

    router
}
