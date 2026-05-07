use std::sync::Arc;

use async_openai::{Client, config::Config};
use axum::{extract::FromRef, http::HeaderMap};
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;
use secrecy::{ExposeSecret, SecretString};

use crate::portkey::PortkeyConfig;

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
