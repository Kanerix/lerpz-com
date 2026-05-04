/// A type alias for [`Result<T, Error>`].
///
/// Used by this crate to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `lerpz-agent` crate might produce.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to build the Portkey/OpenAI gateway client.
    #[error("failed to build LLM client: {0}")]
    ClientBuild(String),

    /// Failed to connect to or query Qdrant.
    #[error("qdrant error: {0}")]
    Qdrant(String),

    /// A component was accessed that was not enabled when building the factory.
    ///
    /// Call the corresponding `with_*` method on [`AgentFactoryBuilder`] to
    /// enable it.
    #[error("`{0}` is not configured on this AgentFactory")]
    NotConfigured(&'static str),

    /// An agent prompt returned an error.
    #[error("completion error: {0}")]
    Completion(#[from] rig::completion::PromptError),
}
