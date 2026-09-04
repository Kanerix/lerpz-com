use crate::state::AppState;

use utoipa_axum::{router::OpenApiRouter, routes};

mod health;
mod runtimes;
mod volumes;

pub fn router(state: AppState) -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/volumes", volumes::router())
        .nest("/runtimes", runtimes::router())
        .routes(routes!(health::handler))
        .with_state(state)
}
