//! Factory for creating pre-configured AI agents.
//!
//! [`AgentFactory`] owns the shared infrastructure (OpenAI client, Qdrant
//! vector store, embedding model, HTTP client) needed to assemble a fully wired
//! agent on every incoming request. Each call to [`AgentFactory::create`]
//! produces a fresh [`Agent`] with both tools attached:

use qdrant_client::Qdrant;
use qdrant_client::qdrant::QueryPoints;
use rig::agent::Agent;
use rig::client::CompletionClient;
use rig::providers::openai;
use rig::providers::openai::responses_api::ResponsesCompletionModel;
use rig_qdrant::QdrantVectorStore;

use crate::tools::{GetUserProfile, SearchKnowledgeBase};

/// Factory for creating AI agents with preconfigured tools.
pub struct AgentFactory {
    /// OpenAI-compatible client used to build completion models.
    client: openai::Client,
    /// Default model identifier forwarded to the completion API.
    model: String,
    /// Qdrant client shared across all agents.
    qdrant: Qdrant,
    /// Embedding model used to vectorise knowledge-base search queries.
    embedding_model: openai::EmbeddingModel,
    /// Base query parameters for Qdrant vector searches (collection name, etc.).
    query_params: QueryPoints,
    /// HTTP client forwarded to the `GetUserProfile` tool.
    http_client: reqwest::Client,
}

impl AgentFactory {
    /// Creates a new [`AgentFactory`].
    pub fn new(
        client: openai::Client,
        model: impl Into<String>,
        qdrant: Qdrant,
        query_params: QueryPoints,
        embedding_model: openai::EmbeddingModel,
        http_client: reqwest::Client,
    ) -> Self {
        Self {
            client,
            model: model.into(),
            qdrant,
            embedding_model,
            query_params,
            http_client,
        }
    }

    /// Creates an agent with preconfigured tools.
    ///
    /// `token` is the caller's Bearer access token. It is forwarded to the
    /// [`GetUserProfile`] tool and **never** sent to the language model.
    pub fn create(&self, token: &str) -> Agent<ResponsesCompletionModel> {
        let store = QdrantVectorStore::new(
            self.qdrant.clone(),
            self.embedding_model.clone(),
            self.query_params.clone(),
        );

        self.client
            .agent(self.model.as_str())
            .preamble(include_str!("prompts/preamble.txt"))
            .tool(SearchKnowledgeBase(store))
            .tool(GetUserProfile::new(self.http_client.clone(), token))
            .build()
    }
}
