//! Application state shared across all axum handlers.

use std::sync::Arc;

use axum::extract::FromRef;
use lerpz_axum::middleware::azure::AzureConfig;

use crate::factory::AgentFactory;

/// Shared application that can by injected into axum handlers via [`State`].
#[derive(Clone)]
pub struct AppState {
    pub azure_config: AzureConfig,
    pub factory: Arc<AgentFactory>,
}

impl AppState {
    pub fn new(azure_config: AzureConfig, factory: AgentFactory) -> Self {
        Self {
            azure_config,
            factory: Arc::new(factory),
        }
    }
}

impl FromRef<AppState> for AzureConfig {
    fn from_ref(state: &AppState) -> Self {
        state.azure_config.clone()
    }
}

impl FromRef<AppState> for Arc<AgentFactory> {
    fn from_ref(state: &AppState) -> Self {
        state.factory.clone()
    }
}
