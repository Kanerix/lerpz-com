//! Tool definitions for the AI agent.
//!
//! Each tool implements the [`rig::tool::Tool`] trait. The agent will
//! automatically serialize/deserialize arguments and dispatch calls to the
//! correct [`Tool::call`] implementation.
//!
//! # Adding a new tool
//!
//! 1. Define an `Args` struct deriving [`serde::Deserialize`] and
//!    [`schemars::JsonSchema`]. Doc-comments on fields become the JSON Schema
//!    `description` the LLM uses to understand each parameter.
//! 2. Define an error type that implements `std::error::Error` (use `thiserror`).
//! 3. Implement [`rig::tool::Tool`] on a unit struct (or a struct holding any
//!    shared state the tool needs), setting `NAME`, `Args`, `Output`, `Error`,
//!    and the async `call` method.
//! 4. Register the tool on the `AgentBuilder` in `agent.rs` via `.tool(MyTool)`.
//!
//! # Why `VectorStoreIndexDyn` for Qdrant searches?
//!
//! `rig-qdrant` uses an internal `QdrantFilter` type for its
//! [`VectorStoreIndex::Filter`] associated type. That type is not publicly
//! exported from the crate, so we cannot construct a
//! `VectorSearchRequest<QdrantFilter>` here. Instead we use the
//! [`VectorStoreIndexDyn`] blanket impl, whose `top_n` method accepts the
//! fully-public `Filter<serde_json::Value>` and handles the conversion to
//! `QdrantFilter` internally.

use rig::completion::ToolDefinition;
use rig::embeddings::EmbeddingModel;
use rig::tool::Tool;
use rig::vector_store::VectorSearchRequest;
use rig::vector_store::VectorStoreIndexDyn;
use rig::vector_store::request::Filter;
use rig_qdrant::QdrantVectorStore;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

// ---------------------------------------------------------------------------
// Shared error type
// ---------------------------------------------------------------------------

/// Errors that can occur when executing any tool.
///
/// Each variant maps to a specific tool so that call-site error messages
/// are precise and actionable.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("knowledge base search failed: {0}")]
    SearchFailed(String),

    #[error("failed to retrieve user profile: {0}")]
    ProfileFailed(String),
}

// ---------------------------------------------------------------------------
// SearchKnowledgeBase
// ---------------------------------------------------------------------------

/// Arguments accepted by the [`SearchKnowledgeBase`] tool.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchKnowledgeBaseArgs {
    /// The natural-language query used to find relevant documents.
    pub query: String,

    /// Maximum number of results to return. Defaults to `5` when omitted.
    pub top_k: Option<u32>,
}

/// Result returned by the [`SearchKnowledgeBase`] tool.
#[derive(Debug, Serialize)]
pub struct SearchKnowledgeBaseOutput {
    pub results: Vec<KnowledgeBaseResult>,
}

/// A single search result from the knowledge base.
#[derive(Debug, Serialize)]
pub struct KnowledgeBaseResult {
    /// Document title extracted from the Qdrant point payload.
    pub title: String,
    /// Document body / chunk extracted from the Qdrant point payload.
    pub content: String,
    /// Cosine similarity score returned by Qdrant (higher = more relevant).
    pub score: f64,
}

/// Search the Qdrant knowledge base for documents relevant to a query.
///
/// The store is injected at construction time by `agent.rs` so that the tool
/// shares the same embedding model and collection as the [`dynamic_context`]
/// RAG pipeline.
pub struct SearchKnowledgeBase<M: EmbeddingModel>(pub QdrantVectorStore<M>);

impl<M> Tool for SearchKnowledgeBase<M>
where
    M: EmbeddingModel + Send + Sync + 'static,
{
    const NAME: &'static str = "search_knowledge_base";

    type Error = ToolError;
    type Args = SearchKnowledgeBaseArgs;
    type Output = SearchKnowledgeBaseOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the knowledge base for documents relevant to the given query. \
                          Always call this tool before answering factual questions so that \
                          answers are grounded in retrieved sources."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The natural-language query used to find relevant documents."
                    },
                    "top_k": {
                        "type": "integer",
                        "description": "Maximum number of results to return (default: 5).",
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "required": ["query"],
                "additionalProperties": false
            }),
        }
    }

    #[instrument(skip(self), fields(tool = Self::NAME))]
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let top_k = args.top_k.unwrap_or(5);

        tracing::debug!(query = %args.query, top_k, "Searching knowledge base");

        // Build the search request using the public `Filter<serde_json::Value>`
        // type. We do NOT set a filter here, so all points in the collection
        // are candidates. Add `.filter(...)` if you need to scope the search to
        // a specific document category or tenant in the future.
        let req = VectorSearchRequest::<Filter<serde_json::Value>>::builder()
            .query(args.query)
            .samples(top_k as u64)
            .build()
            .map_err(|e| ToolError::SearchFailed(e.to_string()))?;

        // `VectorStoreIndexDyn::top_n` accepts `Filter<serde_json::Value>` and
        // handles the conversion to `QdrantFilter` internally. The payload is
        // deserialized as a plain `serde_json::Value` which we extract fields
        // from below.
        //
        // Return type: Vec<(score: f64, id: String, payload: serde_json::Value)>
        let hits = VectorStoreIndexDyn::top_n(&self.0, req)
            .await
            .map_err(|e| ToolError::SearchFailed(e.to_string()))?;

        tracing::debug!(count = hits.len(), "Knowledge base search complete");

        let results = hits
            .into_iter()
            .map(|(score, _id, payload)| {
                // Documents are expected to have at minimum `content` in their
                // Qdrant payload. `title` is optional — fall back to "Untitled"
                // if the indexed document did not include one.
                let title = payload
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Untitled")
                    .to_string();

                let content = payload
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();

                KnowledgeBaseResult {
                    title,
                    content,
                    score,
                }
            })
            .collect();

        Ok(SearchKnowledgeBaseOutput { results })
    }
}

// ---------------------------------------------------------------------------
// GetUserProfile
// ---------------------------------------------------------------------------

/// Arguments accepted by the [`GetUserProfile`] tool.
///
/// This tool takes no parameters — the empty object schema signals to the LLM
/// that it should call the tool without any arguments.
#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetUserProfileArgs {}

/// The user profile returned by [`GetUserProfile`].
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
}

/// Retrieve the current authenticated user's profile information.
pub struct GetUserProfile;

impl Tool for GetUserProfile {
    const NAME: &'static str = "get_user_profile";

    type Error = ToolError;
    type Args = GetUserProfileArgs;
    type Output = UserProfile;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Retrieve the current user's profile information, including their \
                          name and email address."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {},
                "additionalProperties": false
            }),
        }
    }

    #[instrument(skip(self), fields(tool = Self::NAME))]
    async fn call(&self, _args: Self::Args) -> Result<Self::Output, Self::Error> {
        tracing::debug!("Fetching user profile");

        // TODO: replace this stub with a real database / auth-service lookup.
        Ok(UserProfile {
            name: "Kasper".to_string(),
            email: "kas@lerpz.com".to_string(),
        })
    }
}