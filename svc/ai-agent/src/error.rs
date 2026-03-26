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
    #[error("openai error: {0}")]
    OpenAI(#[from] async_openai::error::OpenAIError),
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("generic error: {0}")]
    Generic(#[from] anyhow::Error),
}