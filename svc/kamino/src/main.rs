use crate::config::CONFIG;
use crate::oapi::api_doc;
use crate::portkey::PortkeyConfig;
use crate::state::AppState;

use std::sync::Arc;
use std::time::Duration;

use async_openai::Client;
use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region};
use axum::http::Method;
use axum::response::{Html, IntoResponse, Redirect};
use axum::{Json, routing::get};
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use lerpz_axum::shutdown_signal;
use scalar_api_reference::scalar_html;
use secrecy::{ExposeSecret, SecretString};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa_axum::router::OpenApiRouter;

mod api;
mod config;
mod oapi;
mod portkey;
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

    let portkey_config = PortkeyConfig {
        api_base: CONFIG.PORTKEY_BASE_URL.to_string(),
        api_key: SecretString::from(CONFIG.PORTKEY_API_KEY.clone()),
        api_provider: CONFIG.PORTKEY_PROVIDER.to_string(),
    };
    let openai = Arc::new(Client::with_config(portkey_config));

    let database = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(CONFIG.DATABASE_URL.expose_secret())
        .await
        .unwrap_or_else(|err| panic!("can't connect to database: {err}"));

    let redis_manager = RedisConnectionManager::new(CONFIG.REDIS_URL.expose_secret())
        .unwrap_or_else(|err| panic!("can't connect to redis: {err}"));
    let redis = bb8::Pool::builder()
        .build(redis_manager)
        .await
        .unwrap_or_else(|err| panic!("can't create redis pool: {err}"));

    let aws_credentials = Credentials::new(
        CONFIG.AWS_ACCESS_KEY_ID.as_ref(),
        CONFIG.AWS_SECRET_ACCESS_KEY.as_ref(),
        None,
        None,
        "env",
    );
    let aws_config = aws_sdk_s3::config::Builder::new()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(CONFIG.AWS_REGION.as_ref()))
        .endpoint_url(CONFIG.AWS_S3_ENDPOINT.as_ref())
        .force_path_style(true)
        .credentials_provider(aws_credentials)
        .build();
    let s3 = aws_sdk_s3::Client::from_conf(aws_config);

    let state = AppState {
        azure_config,
        openai,
        database,
        redis,
        s3,
    };

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
