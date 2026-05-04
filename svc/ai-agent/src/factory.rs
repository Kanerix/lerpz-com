//! Service-specific agent factory for the `ai-agent` service.
//!
//! This module wraps [`lerpz_agent::AgentFactory`] and adds the
//! service-specific tools ([`SearchKnowledgeBase`], [`GetUserProfile`]) and
//! preamble to produce a fully-configured per-request agent.

use secrecy::ExposeSecret;

use lerpz_agent::{AgentConfig, AgentFactory as InfraFactory, LerpzAgent, QdrantConfig};

use crate::config::CONFIG;
use crate::tools::{GetUserProfile, SearchKnowledgeBase};

const PREAMBLE: &str = include_str!("prompts/preamble.txt");

/// Per-request agent type used throughout the service.
pub type Agent = LerpzAgent;

/// Service-specific agent factory.
///
/// Wraps [`lerpz_agent::AgentFactory`] and assembles a per-request [`Agent`] in
/// [`build`][AgentFactory::build] by attaching the service preamble and tools.
pub struct AgentFactory(InfraFactory);

impl AgentFactory {
    /// Initialise shared infrastructure from the service [`CONFIG`].
    pub async fn new() -> anyhow::Result<Self> {
        let inner = InfraFactory::builder(AgentConfig {
            portkey_base_url: CONFIG.PORTKEY_BASE_URL.clone(),
            portkey_api_key: secrecy::SecretString::new(
                CONFIG.PORTKEY_API_KEY.expose_secret().to_owned().into(),
            ),
            portkey_provider: CONFIG.PORTKEY_PROVIDER.clone(),
            model: CONFIG.DEFAULT_MODEL.clone(),
        })
        .with_qdrant(QdrantConfig {
            url_grpc: CONFIG.QDRANT_URL_GRPC.clone(),
            collection: CONFIG.QDRANT_COLLECTION.clone(),
            embedding_model: CONFIG.DEFAULT_EMBEDDING_MODEL.clone(),
        })
        .build()
        .await?;

        Ok(Self(inner))
    }

    /// Assemble a per-request [`Agent`] with service-specific tools attached.
    ///
    /// # Errors
    ///
    /// Returns an error if Qdrant was not configured (should not happen in
    /// normal operation since [`new`][AgentFactory::new] always calls
    /// `with_qdrant`).
    ///
    /// `token` is the caller's Azure AD bearer token, forwarded to the
    /// Microsoft Graph API by [`GetUserProfile`].
    pub fn build(&self, token: impl Into<String>) -> anyhow::Result<Agent> {
        let store = self.0.qdrant_store()?;

        Ok(self
            .0
            .agent_builder()
            .preamble(PREAMBLE)
            .tool(SearchKnowledgeBase(store))
            .tool(GetUserProfile::new(self.0.http_client.clone(), token))
            .build())
    }
}
