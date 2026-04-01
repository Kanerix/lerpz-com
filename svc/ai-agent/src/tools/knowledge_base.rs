use rig::completion::ToolDefinition;
use rig::embeddings::EmbeddingModel;
use rig::tool::Tool;
use rig::vector_store::VectorSearchRequest;
use rig::vector_store::VectorStoreIndexDyn;
use rig::vector_store::request::Filter;
use rig_qdrant::QdrantVectorStore;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::instrument;

use crate::tools::ToolError;

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

        tracing::debug!(query = %args.query, top_k, "searching knowledge base");

        let req = VectorSearchRequest::<Filter<Value>>::builder()
            .query(args.query)
            .samples(top_k as u64)
            .build()
            .map_err(|e| ToolError::SearchFailed(e.to_string()))?;

        let hits = VectorStoreIndexDyn::top_n(&self.0, req)
            .await
            .map_err(|e| ToolError::SearchFailed(e.to_string()))?;

        tracing::debug!(count = hits.len(), "knowledge base search complete");

        let results = hits
            .into_iter()
            .map(|(score, _id, payload)| {
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
