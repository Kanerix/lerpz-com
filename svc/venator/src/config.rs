//! Configuration module for the server.

use std::{
    net::SocketAddr,
    sync::{Arc, LazyLock},
};

use axum::http::HeaderValue;
use lerpz_utils::{
    env::{get_env_from, get_env_parse},
    generate_config,
};
use secrecy::SecretString;

/// The environment the server is running in.
#[derive(strum::EnumString, Debug, Clone, Copy, PartialEq, Eq)]
enum Env {
    #[strum(serialize = "production")]
    Production,
    #[strum(serialize = "development")]
    Development,
    #[strum(serialize = "test")]
    Test,
}

/// The main configuration struct for the server.
///
/// Lazy loaded using [`LazyLock`] to ensure that the configuration is only
/// loaded once.
pub static CONFIG: LazyLock<Config> =
    LazyLock::new(|| Config::from_env().expect("failed to load config from environment"));

generate_config!(
    ENV: Env = get_env_parse,
    ADDR: SocketAddr = get_env_parse,
    ALLOWED_ORIGINS: HeaderValue = get_env_parse,
    ENTRA_ID_TENANT_ID: Arc<str> = get_env_from,
    ENTRA_ID_CLIENT_ID: Arc<str> = get_env_from,
    ENTRA_ID_SCOPE: Arc<str> = get_env_from,
    PORTKEY_BASE_URL: Arc<str> = get_env_from,
    PORTKEY_PROVIDER: Arc<str> = get_env_from,
    PORTKEY_API_KEY: SecretString = get_env_from,
    DEFAULT_COMPLETIONS_MODEL: Arc<str> = get_env_from,
    DEFAULT_IMAGE_MODEL: Arc<str> = get_env_from,
    DATABASE_URL: SecretString = get_env_from,
    REDIS_URL: SecretString = get_env_from,
    AWS_ACCESS_KEY_ID: Arc<str> = get_env_from,
    AWS_SECRET_ACCESS_KEY: Arc<str> = get_env_from,
    AWS_REGION: Arc<str> = get_env_from,
    AWS_S3_ENDPOINT: Arc<str> = get_env_from,
    AWS_S3_BUCKET: Arc<str> = get_env_from
);
