use crate::config::CONFIG;
use crate::state::AppState;

use std::time::Duration;

use axum::Router;
use bb8_redis::RedisConnectionManager;
use lerpz_axum::shutdown_signal;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::from(format!("{}=debug,info", env!("CARGO_CRATE_NAME")))
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

    let database_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&CONFIG.DATABASE_URL)
        .await
        .unwrap_or_else(|err| panic!("can't connect to database: {err}"));

    let manager = RedisConnectionManager::new(CONFIG.REDIS_URL.clone())
        .unwrap_or_else(|err| panic!("can't connect to redis: {err}"));
    let redis_pool = bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap_or_else(|err| panic!("can't create redis pool: {err}"));

    let state = AppState {
        database: database_pool,
        redis: redis_pool,
    };

    let app = Router::new()
        .nest("/api", api::router(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&CONFIG.ADDR).await?;
    tracing::info!("server started listening on {}", CONFIG.ADDR);

    let service = app.into_make_service();
    axum::serve(listener, service)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
