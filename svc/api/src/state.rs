use std::sync::Arc;

use async_openai::Client;
use axum::extract::FromRef;
use lerpz_ai::portkey::PortkeyConfig;
use lerpz_axum::middleware::azure::AzureConfig;

use crate::forge::ForgeClient;

pub(crate) type OpenAI = Arc<Client<PortkeyConfig>>;

pub(crate) type DatabasePool = sqlx::PgPool;

pub(crate) type RedisConnection = redis::aio::ConnectionManager;

pub(crate) type S3Client = aws_sdk_s3::Client;

#[derive(Clone)]
pub(crate) struct AppState {
    pub azure_config: AzureConfig,
    pub forge: ForgeClient,
    pub openai: OpenAI,
    pub database: DatabasePool,
    pub redis: RedisConnection,
    pub s3: S3Client,
}

impl FromRef<AppState> for AzureConfig {
    fn from_ref(state: &AppState) -> Self {
        state.azure_config.clone()
    }
}

impl FromRef<AppState> for ForgeClient {
    fn from_ref(state: &AppState) -> Self {
        state.forge.clone()
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

impl FromRef<AppState> for RedisConnection {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}

impl FromRef<AppState> for S3Client {
    fn from_ref(state: &AppState) -> Self {
        state.s3.clone()
    }
}
