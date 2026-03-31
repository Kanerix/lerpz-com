/// Errors that can occur when executing any tool.
///
/// Each variant maps to a specific tool so that call-site error messages are
/// precise and actionable.
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    #[error("knowledge base search failed: {0}")]
    SearchFailed(String),

    #[error("failed to retrieve user profile: {0}")]
    ProfileFailed(String),
}
