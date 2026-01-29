use std::sync::Arc;

use async_openai::{Client, config::Config};
use axum::{extract::FromRef, http::HeaderMap};
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use secrecy::{ExposeSecret, SecretString};
use tokio::sync::RwLock;

type OpenAI = Arc<RwLock<Client<PortkeyConfig>>>;

#[derive(Clone)]
pub struct AppState {
    pub azure_config: AzureConfig,
    pub openai: OpenAI,
    pub database: sqlx::PgPool,
    pub redis: bb8::Pool<RedisConnectionManager>,
}

#[derive(Clone, Debug)]
pub struct PortkeyConfig {
    pub api_base: Arc<str>,
    pub api_key: SecretString,
    pub api_provider: Arc<str>,
}

impl Config for PortkeyConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Content-Type",
            "application/json"
                .parse()
                .expect("Invalid Content-Type header value for PortkeyConfig"),
        );
        headers.insert(
            "x-portkey-api-key",
            self.api_key
                .expose_secret()
                .parse()
                .expect("Invalid x-portkey-api-key header value for PortkeyConfig"),
        );
        headers.insert(
            "x-portkey-provider",
            self.api_provider
                .parse()
                .expect("Invalid x-portkey-config header value for PortkeyConfig"),
        );

        headers
    }

    fn api_key(&self) -> &SecretString {
        &self.api_key
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn query(&self) -> Vec<(&str, &str)> {
        vec![]
    }
}

impl FromRef<AppState> for AzureConfig {
    fn from_ref(state: &AppState) -> Self {
        state.azure_config.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<Client<PortkeyConfig>>> {
    fn from_ref(state: &AppState) -> Self {
        state.openai.clone()
    }
}

impl FromRef<AppState> for sqlx::PgPool {
    fn from_ref(state: &AppState) -> sqlx::PgPool {
        state.database.clone()
    }
}

impl FromRef<AppState> for bb8::Pool<RedisConnectionManager> {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
