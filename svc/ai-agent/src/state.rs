//! Application state shared across all axum handlers.

use std::sync::Arc;

use axum::extract::FromRef;
use lerpz_axum::middleware::azure::AzureConfig;
use rig::agent::{Agent, AgentBuilder};

use crate::factory::AgentFactory;

/// Shared application that can by injected into axum handlers via [`State`].
#[derive(Clone)]
pub struct AppState {
    pub azure_config: AzureConfig,
    pub agent_factory: Arc<AgentFactory>,
}

impl AppState {
    pub fn new(azure_config: AzureConfig, agent_factory: AgentFactory) -> Self {
        Self {
            azure_config,
            agent_factory: Arc::new(agent_factory),
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
        state.agent_factory.clone()
    }
}
