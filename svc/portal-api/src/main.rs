use crate::state::AppState;
use crate::{config::CONFIG, state::PortkeyConfig};

use std::sync::Arc;
use std::time::Duration;

use async_openai::Client;
use axum::Router;
use axum::http::Method;
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::shutdown_signal;
use secrecy::SecretString;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!(
                "{}=debug,lerpz=debug,none",
                env!("CARGO_CRATE_NAME")
            ))
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    #[cfg(debug_assertions)]
    {
        use std::path::PathBuf;

        let env_path = PathBuf::from_iter([env!("CARGO_MANIFEST_DIR"), ".env"]);
        if let Err(err) = dotenvy::from_path(&env_path) {
            tracing::warn!("failed loading .env file: {}", err);
        }
    }

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
    let openai = Arc::new(RwLock::new(Client::with_config(portkey_config)));

    let database = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&CONFIG.DATABASE_URL)
        .await
        .unwrap_or_else(|err| panic!("can't connect to database: {err}"));

    let manager = RedisConnectionManager::new(CONFIG.REDIS_URL.clone())
        .unwrap_or_else(|err| panic!("can't connect to redis: {err}"));
    let redis = bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap_or_else(|err| panic!("can't create redis pool: {err}"));

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

    let app = Router::new()
        .nest("/api/v1", api::router(state.clone()))
        .with_state(state)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
