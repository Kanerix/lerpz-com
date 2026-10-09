use std::future::Future;

use async_openai::{Client, config::Config};

use super::{ChatStream, UpstreamError};

/// A text message in a chat completion.
#[derive(Debug, Clone)]
pub enum ChatMessage {
    /// A message from the user.
    User(String),
    /// A previous reply from the assistant.
    Assistant(String),
}

/// Parameters for a streamed chat completion.
#[derive(Debug, Clone)]
pub struct ChatRequest {
    /// Model (deployment name) to route the request to.
    pub model: String,
    /// Conversation messages in order, including the latest user message.
    pub messages: Vec<ChatMessage>,
    /// Optional reasoning level. Unknown values fall back to `low`.
    pub reasoning: Option<String>,
    /// Optional stable end-user identifier forwarded for abuse monitoring.
    pub user: Option<String>,
}

/// An event emitted while streaming a chat completion.
#[derive(Debug, Clone)]
pub enum ChatEvent {
    /// An incremental chain-of-thought / reasoning chunk.
    Reasoning(String),
    /// An incremental answer chunk.
    Message(String),
    /// The provider's content filter blocked the response.
    Filtered,
}

/// Streams chat completions through a provider.
pub trait ChatGeneration {
    /// The configuration used by this provider's `async-openai` client.
    type Config: Config;

    /// Starts streaming a chat completion for `request`.
    ///
    /// Start-up and stream failures are reported as classified upstream errors.
    fn generate_chat(
        &self,
        client: &Client<Self::Config>,
        request: ChatRequest,
    ) -> impl Future<Output = Result<ChatStream, UpstreamError>> + Send;
}
