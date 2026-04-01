//! Application state shared across all axum handlers.

use std::sync::Arc;

use axum::extract::FromRef;

use crate::agent::Agent;

/// Shared application state injected into every axum handler via [`State`].
///
/// Wrapping the agent in an [`Arc`] allows it to be cheaply cloned for each
/// request without duplicating the underlying model configuration.
#[derive(Clone)]
pub struct AppState {
    pub agent: Arc<Agent>,
}

impl AppState {
    pub fn new(agent: Agent) -> Self {
        Self {
            agent: Arc::new(agent),
        }
    }
}

impl FromRef<AppState> for Arc<Agent> {
    fn from_ref(state: &AppState) -> Self {
        state.agent.clone()
    }
}