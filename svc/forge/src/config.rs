//! Configuration module for the server.

use std::{
    net::SocketAddr,
    num::NonZeroU16,
    sync::{Arc, LazyLock},
};

use axum::http::{HeaderValue, Uri};
use lerpz_utils::{
    env::{get_env_from, get_env_parse},
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

    KUBE_NAMESPACE: Arc<str> = get_env_from,
    AGENT_RUNTIME_IMAGE: Arc<str> = get_env_from,
    AGENT_RUNTIME_SERVICE_ACCOUNT: Arc<str> = get_env_from,
    AGENT_RUNTIME_ORIGIN: Uri = get_runtime_origin,
    AGENT_RUNTIME_PORT: NonZeroU16 = get_env_parse,
    AGENT_RUNTIME_TLS_SECRET: Arc<str> = get_env_from,
    AGENT_MEMORY_STORAGE_CLASS: Arc<str> = get_env_from,
    AGENT_MEMORY_DEFAULT_SIZE: Arc<str> = get_env_from
);

fn get_runtime_origin(key: &str) -> lerpz_utils::env::Result<Uri> {
    let origin: Uri = get_env_parse(key)?;
    let valid_host = origin.host().is_some_and(|host| {
        !host.is_empty()
            && host
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-'))
    });

    if origin.scheme_str() != Some("https")
        || !valid_host
        || origin.path() != "/"
        || origin.query().is_some()
        || origin
            .authority()
            .is_some_and(|authority| authority.as_str().contains('@'))
    {
        return Err(lerpz_utils::env::Error::ParseError(
            key.to_owned(),
            "an HTTPS origin with a DNS host, optional port and no path, query or credentials"
                .to_owned(),
        ));
    }

    Ok(origin)
}
