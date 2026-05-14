//! Application state shared across all axum handlers.

use std::sync::Arc;

use axum::extract::FromRef;
use lerpz_axum::middleware::azure::AzureConfig;

use crate::factory::AgentFactory;

pub type DatabasePool = sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub azure_config: AzureConfig,
    pub agent_factory: Arc<AgentFactory>,
    pub database: DatabasePool,
}

impl AppState {
    pub fn new(
        azure_config: AzureConfig,
        agent_factory: AgentFactory,
        database: sqlx::PgPool,
    ) -> Self {
        Self {
            azure_config,
            agent_factory: Arc::new(agent_factory),
            database,
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

impl FromRef<AppState> for DatabasePool {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}
