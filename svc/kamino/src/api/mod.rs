use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod chats;
mod groups;
mod health;
mod images;
mod models;
mod orgs;

#[cfg(debug_assertions)]
mod failure;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    let router = OpenApiRouter::new()
        .nest("/orgs", orgs::router())
        .nest("/groups", groups::router())
        .nest("/chats", chats::router())
        .nest("/images", images::router())
        .nest("/models", models::router())
        .routes(routes!(health::handler))
        .with_state(state);

    #[cfg(debug_assertions)]
    let router = OpenApiRouter::new()
        .routes(routes!(failure::handler))
        .merge(router);

    router
}
