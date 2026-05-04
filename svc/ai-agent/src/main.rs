use crate::config::CONFIG;
use crate::factory::AgentFactory;
use crate::oapi::ApiDoc;
use crate::state::AppState;

use axum::Json;
use axum::response::{Html, IntoResponse, Redirect};
use axum::routing::get;
use http::Method;
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::shutdown_signal;
use scalar_api_reference::scalar_html;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

mod api;
mod config;
mod oapi;
mod factory;
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

    let azure_config = AzureConfig::new(
        CONFIG.ENTRA_ID_TENANT_ID.clone(),
        CONFIG.ENTRA_ID_CLIENT_ID.clone(),
    )
    .await?;

    let factory = AgentFactory::new().await?;
    let state = AppState::new(azure_config, factory);

    let cors = CorsLayer::new()
        .allow_origin(CONFIG.ALLOWED_ORIGINS.clone())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1", api::router(state.clone()))
        .with_state(state)
        .layer(cors)
        .fallback(redirect)
        .split_for_parts();

    let scalar_config = json!({
        "spec": {
            "url": "/api/openapi.json"
        },
        "authentication": {
            "preferredSecurityScheme": "oauth2",
            "securitySchemes": {
                "oauth2": {
                    "flows": {
                        "authorizationCode": {
                            "x-scalar-client-id": CONFIG.ENTRA_ID_CLIENT_ID,
                            "x-usePkce": "SHA-256",
                            "selectedScopes": [CONFIG.ENTRA_ID_SCOPE]
                        }
                    }
                }
            }
        }
    });

    let html = scalar_html(&scalar_config, None);

    let app = router
        .route("/api/openapi.json", get(|| async { Json(api) }))
        .route("/scalar", get(move || async move { Html(html) }))
        .layer(TraceLayer::new_for_http());

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
