//! Configuration module for the server.

use std::{net::SocketAddr, sync::LazyLock};

use lerpz_utils::{
    env::{get_env, get_env_parse},
    generate_config,
};

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
/// Lazy loaded using `LazyLock` to ensure that the configuration is only loaded
/// once.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

generate_config!(
    ENV: Env = get_env_parse,
    ADDR: SocketAddr = get_env_parse,
    ENTRA_ID_TENANT_ID: String = get_env,
    ENTRA_ID_CLIENT_ID: String = get_env,
    PORTKEY_BASE_URL: String = get_env,
    PORTKEY_PROVIDER: String = get_env,
    PORTKEY_API_KEY: String = get_env,
    DATABASE_URL: String = get_env,
    REDIS_URL: String = get_env
);
