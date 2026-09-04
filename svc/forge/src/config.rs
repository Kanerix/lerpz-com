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
    // The namespace every agent workload is provisioned into. Forge is
    // deliberately scoped to a single namespace so its RBAC can be a namespaced
    // Role rather than a ClusterRole.
    KUBE_NAMESPACE: Arc<str> = get_env_from,
    // Container image used for an agent runtime when the request does not pin
    // one explicitly.
    AGENT_RUNTIME_IMAGE: Arc<str> = get_env_from,
    // ServiceAccount the agent runtime pods run as. This should *not* be the
    // ServiceAccount Forge itself uses, so a compromised agent container cannot
    // reach the Kubernetes API.
    AGENT_RUNTIME_SERVICE_ACCOUNT: Arc<str> = get_env_from,
    // StorageClass backing agent memory volumes.
    AGENT_MEMORY_STORAGE_CLASS: Arc<str> = get_env_from,
    // Default size requested for a memory volume when the request omits one.
    AGENT_MEMORY_DEFAULT_SIZE: Arc<str> = get_env_from
);
