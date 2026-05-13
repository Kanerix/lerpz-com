use std::sync::Arc;

use async_openai::Client;
use axum::extract::FromRef;
use bb8_redis::RedisConnectionManager;
use lerpz_axum::middleware::azure::AzureConfig;

use crate::portkey::PortkeyConfig;

pub(crate) type OpenAI = Arc<Client<PortkeyConfig>>;

pub(crate) type DatabasePool = sqlx::PgPool;

pub(crate) type RedisPool = bb8::Pool<RedisConnectionManager>;

pub(crate) type S3Client = aws_sdk_s3::Client;

#[derive(Clone)]
pub(crate) struct AppState {
    pub azure_config: AzureConfig,
    pub openai: OpenAI,
    pub database: DatabasePool,
    pub redis: RedisPool,
    pub s3: S3Client,
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

impl FromRef<AppState> for S3Client {
    fn from_ref(state: &AppState) -> Self {
        state.s3.clone()
    }
}
