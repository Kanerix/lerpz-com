use crate::config::CONFIG;
use crate::oapi::ApiDoc;
use crate::state::AppState;

use axum::Json;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use lerpz_axum::shutdown_signal;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

mod agent;
mod api;
mod config;
mod error;
mod oapi;
mod portkey;
mod state;
mod tools;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        use std::path::PathBuf;
        let env_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".env"]);
        let _ = dotenvy::from_path(&env_path);
    }

    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!(
                "{}=debug,lerpz=debug,none",
                env!("CARGO_CRATE_NAME")
            ))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let agent = agent::build_agent().await?;
    let state = AppState::new(agent);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", api::router(state.clone()))
        .with_state(state)
        .layer(cors)
        .fallback(redirect)
        .split_for_parts();

    let scalar_html = include_str!("../scalar.html")
        .replace("$client_id", &CONFIG.ENTRA_ID_CLIENT_ID)
        .replace("$scope", &CONFIG.ENTRA_ID_SCOPE);

    let openapi_json = api.clone();
    let app = router
        .route("/api/openapi.json", get(|| async { Json(openapi_json) }))
        .merge(Scalar::with_url("/scalar", api).custom_html(scalar_html));

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

#[axum::debug_handler]
pub async fn redirect() -> impl IntoResponse {
    Redirect::to("/scalar")
}
