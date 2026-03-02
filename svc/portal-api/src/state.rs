use std::sync::Arc;

use async_openai::{Client, config::Config};
use axum::{extract::FromRef, http::HeaderMap};
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use secrecy::{ExposeSecret, SecretString};

pub(crate) type OpenAI = Arc<Client<PortkeyConfig>>;

pub(crate) type DatabasePool = sqlx::PgPool;

pub(crate) type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(Clone)]
pub(crate) struct AppState {
    pub azure_config: AzureConfig,
    pub openai: OpenAI,
    pub database: DatabasePool,
    pub redis: RedisPool,
}

#[derive(Clone, Debug)]
pub(crate) struct PortkeyConfig {
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

impl FromRef<AppState> for OpenAI {
    fn from_ref(state: &AppState) -> Self {
        state.openai.clone()
    }
}

impl FromRef<AppState> for DatabasePool {
    fn from_ref(state: &AppState) -> sqlx::PgPool {
        state.database.clone()
    }
}

impl FromRef<AppState> for RedisPool {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
