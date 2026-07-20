use crate::state::AppState;

use async_openai::types::chat::ReasoningEffort;
use utoipa_axum::{router::OpenApiRouter, routes};

mod create;
mod delete;
mod edit;
mod list;
mod message;
mod message_delete;
mod read;
mod stream;
mod update;

/// Parses a reasoning level string into a [`ReasoningEffort`].
///
/// Recognised levels are `none`, `minimal`, `low`, `medium`, `high` and
/// `xhigh` (case-insensitive). Any unrecognised value falls back to
/// [`ReasoningEffort::Low`].
pub(super) fn parse_reasoning_effort(value: &str) -> ReasoningEffort {
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

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(list::handler))
        .routes(routes!(create::handler))
        .routes(routes!(read::handler))
        .routes(routes!(message::handler))
        .routes(routes!(message_delete::handler))
        .routes(routes!(edit::handler))
        .routes(routes!(update::handler))
        .routes(routes!(delete::handler))
}
