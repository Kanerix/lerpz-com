use crate::oapi::ApiDoc;
use crate::state::AppState;
use crate::{config::CONFIG, state::PortkeyConfig};

use std::sync::Arc;
use std::time::Duration;

use async_openai::Client;
use axum::http::Method;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Json, routing::get};
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::shutdown_signal;
use scalar_api_reference::axum::router as scalar_router;
use scalar_api_reference::scalar_html;
use secrecy::{ExposeSecret, SecretString};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

mod api;
mod config;
mod oapi;
mod state;
mod utils;

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

    let portkey_config = PortkeyConfig {
        api_base: CONFIG.PORTKEY_BASE_URL.clone(),
        api_key: SecretString::from(CONFIG.PORTKEY_API_KEY.clone()),
        api_provider: CONFIG.PORTKEY_PROVIDER.clone(),
    };
    let openai = Arc::new(Client::with_config(portkey_config));

    let database = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(CONFIG.DATABASE_URL.expose_secret())
        .await
        .unwrap_or_else(|err| panic!("can'''t connect to database: {err}"));

    sqlx::migrate!("../../migrations")
        .run(&database)
        .await
        .unwrap_or_else(|err| panic!("can'''t run database migrations: {err}"));

    let redis_manager = RedisConnectionManager::new(CONFIG.REDIS_URL.expose_secret())
        .unwrap_or_else(|err| panic!("can'''t connect to redis: {err}"));
    let redis = bb8::Pool::builder()
        .build(redis_manager)
        .await
        .unwrap_or_else(|err| panic!("can'''t create redis pool: {err}"));

    let state = AppState {
        azure_config,
        openai,
        database,
        redis,
    };

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
        .merge(scalar_router("/scalar", &scalar_config))
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
