//! Default chat completion streaming.
//!
//! Reasoning models routed through the gateway emit their chain-of-thought in
//! one of two shapes the typed `async-openai` stream drops: a flat
//! `reasoning_content` (a.k.a. `reasoning`) delta field, or Anthropic-style
//! incremental `content_blocks` carrying `thinking` deltas. We therefore use
//! the `byot` ("bring your own type") API with a minimal [`StreamChunk`] type
//! that preserves both.

use std::pin::Pin;

use async_openai::{
    Client,
    error::OpenAIError,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequest,
        CreateChatCompletionRequestArgs, ReasoningEffort,
    },
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};

use crate::{
    generation::{
        ChatEvent, ChatGeneration, ChatMessage, ChatRequest, ChatStream, Family, UpstreamError,
        classify_error,
    },
    portkey::PortkeyConfig,
};

/// A minimal view of a streamed chat completion chunk.
///
/// Unknown fields are ignored, so this stays forward-compatible with whatever
/// else the provider includes in each chunk.
#[derive(Debug, Deserialize)]
struct StreamChunk {
    #[serde(default)]
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    #[serde(default)]
    delta: StreamDelta,
    #[serde(default)]
    finish_reason: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
struct StreamDelta {
    /// Incremental answer tokens.
    #[serde(default)]
    content: Option<String>,
    /// Incremental reasoning/chain-of-thought tokens. Providers disagree on the
    /// field name, so accept both `reasoning_content` and `reasoning`.
    #[serde(default, alias = "reasoning")]
    reasoning_content: Option<String>,
    /// Anthropic-style streaming delivers incremental reasoning and answer text
    /// as `content_blocks` instead of `reasoning_content`. We read the
    /// `thinking` deltas from here; the answer text is already mirrored in
    /// `content`, so we ignore the `text` blocks to avoid duplicating it.
    #[serde(default)]
    content_blocks: Vec<ContentBlock>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(default)]
    delta: ContentBlockDelta,
}

#[derive(Debug, Default, Deserialize)]
struct ContentBlockDelta {
    /// Incremental reasoning/chain-of-thought tokens.
    #[serde(default)]
    thinking: Option<String>,
}

type ChunkStream = Pin<Box<dyn Stream<Item = Result<StreamChunk, OpenAIError>> + Send>>;

impl ChatGeneration for Family {
    type Config = PortkeyConfig;

    async fn generate_chat(
        &self,
        client: &Client<Self::Config>,
        request: ChatRequest,
    ) -> Result<ChatStream, UpstreamError> {
        let request = create_request(request)?;
        tracing::trace!(
            model = ?request.model,
            reasoning = ?request.reasoning_effort,
            "starting chat stream"
        );

        let stream = client
            .chat()
            .create_stream_byot::<_, StreamChunk>(request)
            .await
            .map_err(|err| classify_error(&err.to_string()))?;

        Ok(chat_events(stream))
    }
}

fn create_request(request: ChatRequest) -> Result<CreateChatCompletionRequest, UpstreamError> {
    let messages: Vec<ChatCompletionRequestMessage> = request
        .messages
        .into_iter()
        .map(|message| {
            let result = match message {
                ChatMessage::User(content) => ChatCompletionRequestUserMessageArgs::default()
                    .content(content)
                    .build()
                    .map(Into::into),
                ChatMessage::Assistant(content) => {
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(content)
                        .build()
                        .map(Into::into)
                }
            };
            result.map_err(|err| UpstreamError::provider(err.to_string()))
        })
        .collect::<Result<_, _>>()?;

    let mut builder = CreateChatCompletionRequestArgs::default();
    builder.model(request.model).messages(messages).stream(true);

    if let Some(reasoning) = request.reasoning {
        builder.reasoning_effort(parse_reasoning_effort(&reasoning));
    }
    if let Some(user) = request.user {
        builder.user(user);
    }

    builder
        .build()
        .map_err(|err| UpstreamError::provider(err.to_string()))
}

fn parse_reasoning_effort(value: &str) -> ReasoningEffort {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => ReasoningEffort::None,
        "minimal" => ReasoningEffort::Minimal,
        "low" => ReasoningEffort::Low,
        "medium" => ReasoningEffort::Medium,
        "high" => ReasoningEffort::High,
        "xhigh" => ReasoningEffort::Xhigh,
        _ => ReasoningEffort::Low,
    }
}

/// Adapts the raw chunk stream into a stream of [`ChatEvent`]s.
///
/// Reasoning and answer deltas are aggregated per chunk (across choices) before
/// being emitted, matching how the provider batches them.
fn chat_events(mut stream: ChunkStream) -> ChatStream {
    Box::pin(async_stream::stream! {
        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(chunk) => chunk,
                Err(err) => {
                    yield Err(classify_error(&err.to_string()));
                    break;
                }
            };

            let mut content = String::new();
            let mut reasoning = String::new();
            let mut filtered = false;

            for choice in &chunk.choices {
                if let Some(delta) = &choice.delta.reasoning_content {
                    reasoning.push_str(delta);
                }
                for block in &choice.delta.content_blocks {
                    if let Some(delta) = &block.delta.thinking {
                        reasoning.push_str(delta);
                    }
                }
                if let Some(delta) = &choice.delta.content {
                    content.push_str(delta);
                }
                if choice.finish_reason.as_deref() == Some("content_filter") {
                    filtered = true;
                }
            }

            if !reasoning.is_empty() {
                yield Ok(ChatEvent::Reasoning(reasoning));
            }
            if !content.is_empty() {
                yield Ok(ChatEvent::Message(content));
            }
            if filtered {
                yield Ok(ChatEvent::Filtered);
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn builds_stream_request_with_history_and_options() {
        let request = create_request(ChatRequest {
            model: "example-model".to_string(),
            messages: vec![
                ChatMessage::User("hello".to_string()),
                ChatMessage::Assistant("hi".to_string()),
                ChatMessage::User("how are you?".to_string()),
            ],
            reasoning: Some(" HIGH ".to_string()),
            user: Some("user-123".to_string()),
        })
        .expect("valid chat request");
        let value = serde_json::to_value(request).expect("serializable chat request");

        assert_eq!(value["model"], "example-model");
        assert_eq!(value["stream"], true);
        assert_eq!(value["reasoning_effort"], "high");
        assert_eq!(value["user"], "user-123");
        assert_eq!(
            value["messages"],
            json!([
                {"role": "user", "content": "hello"},
                {"role": "assistant", "content": "hi"},
                {"role": "user", "content": "how are you?"}
            ])
        );
    }

    #[test]
    fn preserves_default_reasoning_behaviour() {
        let request = create_request(ChatRequest {
            model: "example-model".to_string(),
            messages: vec![ChatMessage::User("hello".to_string())],
            reasoning: None,
            user: None,
        })
        .expect("valid chat request");
        let value = serde_json::to_value(request).expect("serializable chat request");

        assert!(value.get("reasoning_effort").is_none());
        assert!(value.get("user").is_none());
        assert_eq!(parse_reasoning_effort("unknown"), ReasoningEffort::Low);
    }
}
