use crate::config::CONFIG;
use crate::oapi::api_doc;
use crate::state::AppState;

use axum::http::Method;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Json, routing::get};
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::middleware::instance::CaptureInstanceLayer;
use lerpz_axum::shutdown_signal;
use scalar_api_reference::scalar_html;
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_axum::router::OpenApiRouter;

mod api;
mod config;
mod oapi;
mod resources;
mod state;

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
                "off,{}=debug,lerpz=debug",
                env!("CARGO_CRATE_NAME")
            ))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let azure_config = AzureConfig::new(
        CONFIG.ENTRA_ID_TENANT_ID.as_ref(),
        CONFIG.ENTRA_ID_CLIENT_ID.as_ref(),
    )
    .await?;

    // Resolves the in-cluster ServiceAccount when running as a pod, and falls
    // back to the current kubeconfig context locally. Failing fast here is
    // deliberate: a Forge that cannot reach the API server has nothing to do.
    let kube = kube::Client::try_default()
        .await
        .unwrap_or_else(|err| panic!("can't connect to the kubernetes api: {err}"));

    tracing::info!(
        namespace = %CONFIG.KUBE_NAMESPACE,
        "connected to the kubernetes api"
    );

    let state = AppState {
        azure_config,
        kube,
    };

    let cors = CorsLayer::new()
        .allow_origin(CONFIG.ALLOWED_ORIGINS.clone())
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::with_openapi(api_doc())
        .nest("/api/v1", api::router(state.clone()))
        .with_state(state)
        .layer(CaptureInstanceLayer)
        .layer(cors)
        .fallback(redirect)
        .split_for_parts();

    let scalar_config = json!({
        "url": "/api/openapi.json",
        "favicon": "/favicon.svg",
        "authentication": {
            "preferredSecurityScheme": "oauth2",
            "securitySchemes": {
                "oauth2": {
                    "flows": {
                        "authorizationCode": {
                            "x-scalar-client-id": CONFIG.ENTRA_ID_CLIENT_ID.as_ref(),
                            "x-usePkce": "SHA-256",
                            "selectedScopes": [CONFIG.ENTRA_ID_SCOPE.as_ref()]
                        }
                    }
                }
            }
        }
    });

    let html = scalar_html(&scalar_config, None).replace(
        "<title>Scalar API Reference</title>",
        "<title>Lerpz Forge — API references</title>",
    );

    let app = router
        .route("/api/openapi.json", get(|| async { Json(api) }))
        .route("/scalar", get(move || async move { Html(html) }))
        .route("/favicon.svg", get(favicon))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Redirects to the Scalar API reference.
///
/// This is used as a fallback route to redirect to the Scalar API reference.
#[axum::debug_handler]
pub async fn redirect() -> impl IntoResponse {
    Redirect::to("/scalar")
}

/// Serves the favicon shown in the Scalar API reference tab.
#[axum::debug_handler]
pub async fn favicon() -> impl IntoResponse {
    const FAVICON: &[u8] = include_bytes!("../assets/lerpz.svg");
    (
        [(axum::http::header::CONTENT_TYPE, "image/svg+xml")],
        FAVICON,
    )
}
