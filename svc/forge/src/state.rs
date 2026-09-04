use axum::extract::FromRef;
use lerpz_axum::middleware::azure::AzureConfig;

/// The in-cluster Kubernetes API client.
///
/// [`kube::Client`] is cheap to clone (it wraps an `Arc`ed HTTP service), so it
/// is shared by value rather than behind another layer of indirection.
pub(crate) type KubeClient = kube::Client;

#[derive(Clone)]
pub(crate) struct AppState {
    pub azure_config: AzureConfig,
    pub kube: KubeClient,
}

impl FromRef<AppState> for AzureConfig {
    fn from_ref(state: &AppState) -> Self {
        state.azure_config.clone()
    }
}

impl FromRef<AppState> for KubeClient {
    fn from_ref(state: &AppState) -> Self {
        state.kube.clone()
    }
}
