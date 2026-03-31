//! Agent construction and execution.
//!
//! This module wires together:
//!
//! - A single [`openai::Client`] pointed at the **Portkey** gateway, used for
//!   both chat-completion inference and embedding generation.
//! - Two [`QdrantVectorStore`] instances sharing the same collection, embedding
//!   model, and Qdrant connection:
//!   - `context_store` → passed to [`AgentBuilder::dynamic_context`] so rig
//!     automatically injects the top-N most relevant documents before every
//!     prompt.
//!   - `tool_store` → injected into [`SearchKnowledgeBase`] so the LLM can
//!     also trigger an explicit search at any point during a conversation.
//! - Static tools ([`SearchKnowledgeBase`], [`GetUserProfile`]) registered on
//!   the [`AgentBuilder`].

use qdrant_client::Qdrant;
use qdrant_client::qdrant::QueryPointsBuilder;
use rig::agent::AgentBuilder;
use rig::client::CompletionClient;
use rig::client::EmbeddingsClient;
use rig::completion::Prompt;
use rig::providers::openai;
use rig_qdrant::QdrantVectorStore;
use tracing::instrument;

use crate::config::CONFIG;
use crate::error::{Error, Result};
use crate::portkey::build_portkey_client;
use crate::tools::{GetUserProfile, SearchKnowledgeBase};

/// How many documents to retrieve from the knowledge base for RAG context.
///
/// Increase for broader recall at the cost of a larger context window.
/// Decrease to reduce latency and token usage.
const RAG_TOP_N: usize = 5;

/// A fully configured, ready-to-use rig [`rig::agent::Agent`].
///
/// The inner type is intentionally opaque — callers only need [`Prompt`].
///
/// Uses the OpenAI Responses API completion model, which is what the default
/// `openai::Client` (and any Portkey-proxied variant of it) exposes via
/// [`CompletionClient::completion_model`].
pub type Agent = rig::agent::Agent<openai::responses_api::ResponsesCompletionModel>;

/// Build a production-ready [`Agent`] from the provided [`Config`].
///
/// # Steps
///
/// 1. Builds a single OpenAI-compatible Portkey client for both completions
///    and embeddings. No separate `OPENAI_API_KEY` is required.
/// 2. Connects to Qdrant and creates two [`QdrantVectorStore`] instances from
///    cloned inner components (see module docs for rationale).
/// 3. Assembles a rig [`Agent`] with a system prompt, automatic RAG context
///    injection via `dynamic_context`, and explicit tool support.
///
/// # Errors
///
/// Returns an error if the Portkey client cannot be built (invalid header
/// values) or if the Qdrant client fails to initialise.
#[instrument()]
pub async fn build_agent() -> Result<Agent> {
    let portkey_client = build_portkey_client(
        &CONFIG.PORTKEY_BASE_URL,
        &CONFIG.PORTKEY_API_KEY,
        &CONFIG.PORTKEY_PROVIDER,
    )?;

    let embed_model = portkey_client.embedding_model(CONFIG.DEFAULT_EMBEDDING_MODEL.as_ref());

    let qdrant = Qdrant::from_url(CONFIG.QDRANT_URL_GRPC.as_ref())
        .build()
        .map_err(|e| {
            Error::Agent(format!(
                "failed to connect to Qdrant at {}: {e}",
                CONFIG.QDRANT_URL_GRPC
            ))
        })?;

    let query_params = QueryPointsBuilder::new(CONFIG.QDRANT_COLLECTION.as_ref())
        .with_payload(true)
        .build();

    let tool_store =
        QdrantVectorStore::new(qdrant.clone(), embed_model.clone(), query_params.clone());
    let context_store = QdrantVectorStore::new(qdrant, embed_model, query_params);

    let completion_model = portkey_client.completion_model(CONFIG.DEFAULT_MODEL.as_ref());

    let agent = AgentBuilder::new(completion_model)
        .preamble(
            "You are a helpful assistant for the Lerpz platform. \
             Before answering factual questions, use the search_knowledge_base \
             tool to retrieve relevant information from the knowledge base. \
             Always ground your answers in retrieved sources and cite them \
             where appropriate. \
             Use the get_user_profile tool when the user asks about their \
             account, name, or email.",
        )
        .dynamic_context(RAG_TOP_N, context_store)
        .tool(SearchKnowledgeBase(tool_store))
        .tool(GetUserProfile)
        .build();

    tracing::info!(
        model             = %CONFIG.DEFAULT_MODEL,
        embedding_model   = %CONFIG.DEFAULT_EMBEDDING_MODEL,
        qdrant_collection = %CONFIG.QDRANT_COLLECTION,
        rag_top_n         = RAG_TOP_N,
        "Agent built successfully",
    );

    Ok(agent)
}
