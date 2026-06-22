use std::time::Duration;

use axum::Json;
use axum::response::Html;
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use http::Method;
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::shutdown_signal;
use qdrant_client::Qdrant;
use qdrant_client::qdrant::QueryPointsBuilder;
use rig_core::client::EmbeddingsClient;
use scalar_api_reference::scalar_html;

use secrecy::ExposeSecret;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_axum::router::OpenApiRouter;

use crate::client::build_client;
use crate::config::CONFIG;
use crate::factory::AgentFactory;
use crate::oapi::api_doc;
use crate::state::AppState;

mod api;
mod client;
mod config;
mod factory;
mod oapi;
mod state;
mod stream;
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
        CONFIG.ENTRA_ID_TENANT_ID.as_ref(),
        CONFIG.ENTRA_ID_CLIENT_ID.as_ref(),
    )
    .await?;

    let agent_client = build_client(
        &CONFIG.PORTKEY_BASE_URL,
        &CONFIG.PORTKEY_API_KEY,
        &CONFIG.PORTKEY_PROVIDER,
    )?;

    let default_model = CONFIG.DEFAULT_MODEL.as_ref();

    let qdrant = Qdrant::from_url(&CONFIG.QDRANT_URL_GRPC)
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build Qdrant client: {e}"))?;
    let embedding_model = agent_client.embedding_model(&*CONFIG.DEFAULT_EMBEDDING_MODEL);
    let query_params = QueryPointsBuilder::new(&*CONFIG.QDRANT_COLLECTION).build();

    let http_client = reqwest::Client::new();

    let agent_factory = AgentFactory::new(
        agent_client,
        default_model,
        qdrant,
        query_params,
        embedding_model,
        http_client,
    );

    let database = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(CONFIG.DATABASE_URL.expose_secret())
        .await
        .unwrap_or_else(|err| panic!("can't connect to database: {err}"));

    let state = AppState::new(azure_config, agent_factory, database);

    let cors = CorsLayer::new()
        .allow_origin(CONFIG.ALLOWED_ORIGINS.clone())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    let (router, api) = OpenApiRouter::with_openapi(api_doc())
        .nest("/api/v1", api::router(state.clone()))
        .with_state(state)
        .layer(cors)
        .fallback(redirect)
        .split_for_parts();

    let scalar_config = json!({
        "url": "/api/openapi.json",
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

/// Redirects to the Scalar API reference.
///
/// This is used as a fallback route to redirect to the Scalar API reference.
#[axum::debug_handler]
pub async fn redirect() -> impl IntoResponse {
    Redirect::to("/scalar")
}
