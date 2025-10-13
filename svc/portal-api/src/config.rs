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
}

/// The main configuration struct for the server.
///
/// Lazy loaded using `LazyLock` to ensure that the configuration is only loaded
/// once.
pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::from_env().unwrap());

generate_config!(
    ENV: Env = get_env_parse,
    ADDR: SocketAddr = get_env_parse,
    DATABASE_URL: String = get_env,
    REDIS_URL: String = get_env
);
