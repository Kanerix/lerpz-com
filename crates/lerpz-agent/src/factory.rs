//! Agent factory – constructs per-request agents from shared infrastructure.
//!
//! [`AgentFactory`] is built via [`AgentFactoryBuilder`] (obtained from
//! [`AgentFactory::builder`]). Only the core LLM configuration is required;
//! optional components such as Qdrant are enabled with dedicated builder
//! methods.
//!
//! # Example
//!
//! ```ignore
//! let factory = AgentFactory::builder(AgentConfig { .. })
//!     .with_qdrant(QdrantConfig { .. })
//!     .build()
//!     .await?;
//! ```

use std::sync::Arc;

use qdrant_client::Qdrant;
use qdrant_client::qdrant::{QueryPoints, QueryPointsBuilder};
use reqwest::Client as HttpClient;
use rig::agent::AgentBuilder;
use rig::client::{CompletionClient, EmbeddingsClient};
use rig::providers::openai;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig_qdrant::QdrantVectorStore;
use secrecy::SecretString;

use crate::client::build_client;
use crate::{Error, Result};

/// Core LLM configuration – always required.
///
/// Pass to [`AgentFactory::builder`] to start constructing a factory.
pub struct AgentConfig {
    /// Base URL of the Portkey gateway (e.g. `"https://api.portkey.ai/v1"`).
    pub portkey_base_url: Arc<str>,
    /// Portkey workspace API key.
    pub portkey_api_key: SecretString,
    /// Upstream provider name as understood by Portkey (e.g. `"openai"`).
    pub portkey_provider: Arc<str>,
    /// Completion model identifier (e.g. `"gpt-4o"`).
    pub model: Arc<str>,
}

/// Optional Qdrant / vector-search configuration.
///
/// Pass to [`AgentFactoryBuilder::with_qdrant`] to enable vector-search
/// support on the factory.
pub struct QdrantConfig {
    /// gRPC URL of the Qdrant instance (e.g. `"http://localhost:6334"`).
    pub url_grpc: Arc<str>,
    /// Name of the Qdrant collection to query.
    pub collection: Arc<str>,
    /// Embedding model identifier (e.g. `"text-embedding-3-small"`).
    pub embedding_model: Arc<str>,
}

/// Bundled Qdrant infrastructure. Present only when [`QdrantConfig`] was
/// supplied to the builder.
struct QdrantInfra {
    qdrant: Qdrant,
    embed_model: openai::EmbeddingModel,
    query_params: QueryPoints,
}

/// Builder for [`AgentFactory`].
///
/// Obtain via [`AgentFactory::builder`], optionally chain [`with_qdrant`],
/// then call [`build`] to initialise all configured backends.
///
/// [`with_qdrant`]: AgentFactoryBuilder::with_qdrant
/// [`build`]: AgentFactoryBuilder::build
pub struct AgentFactoryBuilder {
    config: AgentConfig,
    qdrant_config: Option<QdrantConfig>,
}

impl AgentFactoryBuilder {
    fn new(config: AgentConfig) -> Self {
        Self {
            config,
            qdrant_config: None,
        }
    }

    /// Enable Qdrant vector-search support.
    ///
    /// After calling this, [`AgentFactory::qdrant_store`] will return a live
    /// store instead of [`Error::NotConfigured`].
    pub fn with_qdrant(mut self, config: QdrantConfig) -> Self {
        self.qdrant_config = Some(config);
        self
    }

    /// Build the [`AgentFactory`], connecting to all configured backends.
    ///
    /// This is the only `async` step – it performs the Qdrant health-check
    /// (when enabled) and constructs the HTTP connection pool.
    pub async fn build(self) -> crate::Result<AgentFactory> {
        let client = build_client(
            &self.config.portkey_base_url,
            &self.config.portkey_api_key,
            &self.config.portkey_provider,
        )?;

        let completion_model = client.completion_model(self.config.model.as_ref());

        let qdrant = match self.qdrant_config {
            Some(cfg) => {
                let embed_model = client.embedding_model(cfg.embedding_model.as_ref());

                let qdrant = Qdrant::from_url(cfg.url_grpc.as_ref())
                    .build()
                    .map_err(|e| Error::Qdrant(e.to_string()))?;

                let query_params = QueryPointsBuilder::new(cfg.collection.as_ref()).build();

                Some(QdrantInfra {
                    qdrant,
                    embed_model,
                    query_params,
                })
            }
            None => None,
        };

        Ok(AgentFactory {
            completion_model,
            http_client: HttpClient::new(),
            qdrant,
        })
    }
}

///
/// Built via [`AgentFactory::builder`]. All internal handles are Arc-backed
/// and cheap to clone, so the cost of each service-side `build()` call is
/// minimal.
pub struct AgentFactory {
    /// Completion model handle – cheap to clone.
    pub completion_model: ResponsesCompletionModel,
    /// Shared HTTP client – always present, cheap to create.
    pub http_client: HttpClient,
    /// Qdrant infrastructure – only present when [`with_qdrant`] was called.
    ///
    /// [`with_qdrant`]: AgentFactoryBuilder::with_qdrant
    qdrant: Option<QdrantInfra>,
}

impl AgentFactory {
    /// Create a new builder starting from the required core LLM config.
    pub fn builder(config: AgentConfig) -> AgentFactoryBuilder {
        AgentFactoryBuilder::new(config)
    }

    /// Returns a new [`AgentBuilder`] pre-configured with this factory's
    /// completion model.
    ///
    /// Services call this inside their own `build()` method, then attach
    /// tools and a preamble before calling `.build()`.
    pub fn agent_builder(&self) -> AgentBuilder<ResponsesCompletionModel> {
        AgentBuilder::new(self.completion_model.clone())
    }

    /// Returns a new [`QdrantVectorStore`] backed by this factory's Qdrant
    /// client, embedding model, and query parameters.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NotConfigured`] if [`with_qdrant`] was not called
    /// when building this factory.
    ///
    /// [`with_qdrant`]: AgentFactoryBuilder::with_qdrant
    pub fn qdrant_store(&self) -> Result<QdrantVectorStore<openai::EmbeddingModel>> {
        let infra = self.qdrant.as_ref().ok_or(Error::NotConfigured("Qdrant"))?;

        Ok(QdrantVectorStore::new(
            infra.qdrant.clone(),
            infra.embed_model.clone(),
            infra.query_params.clone(),
        ))
    }
}
