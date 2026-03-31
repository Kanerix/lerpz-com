/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors this [`crate`] might produce.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("agent error: {0}")]
    Agent(String),

    #[error("rig completion error: {0}")]
    Rig(#[from] rig::completion::PromptError),

    #[error("vector store error: {0}")]
    VectorStore(#[from] rig::vector_store::VectorStoreError),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("tool error: {0}")]
    Tool(#[from] crate::tools::ToolError),

    #[error("generic error: {0}")]
    Generic(#[from] anyhow::Error),
}
