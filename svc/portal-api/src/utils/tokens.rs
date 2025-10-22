use async_openai::types::CompletionUsage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUsage {
    total_tokens: u32,
}

impl From<CompletionUsage> for TokenUsage {
    fn from(value: CompletionUsage) -> Self {
        TokenUsage {
            total_tokens: value.total_tokens,
        }
    }
}
