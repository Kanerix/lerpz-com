//! Prompt enhancement endpoints.
//!
//! These routes take a rough, user-written prompt and rewrite it into a richer,
//! model-ready prompt tailored to a specific medium (chat, image or video).
//! Unlike the chat and image endpoints, enhancement is a single-shot
//! transformation, so the enhanced prompt is returned as a plain JSON body
//! rather than streamed.

use async_openai::types::chat::{
    ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
};
use axum::http::StatusCode;
use lerpz_axum::problem::{HandlerResult, Problem};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{config::CONFIG, state::OpenAI};

mod chat;
mod image;
mod video;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EnhanceRequest {
    /// The raw prompt to enhance.
    prompt: String,
    /// Optional model override.
    #[serde(default)]
    model: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EnhanceResponse {
    /// The enhanced prompt.
    prompt: String,
}

pub fn router() -> OpenApiRouter<crate::state::AppState> {
    OpenApiRouter::new()
        .routes(routes!(chat::handler))
        .routes(routes!(image::handler))
        .routes(routes!(video::handler))
}

/// Rewrites `prompt` using `system_prompt` as the guiding instruction and
/// returns the enhanced prompt.
///
/// The enhancement runs as a single, non-streaming chat completion. The reply
/// is trimmed and validated to be non-empty before being returned.
async fn enhance_prompt(
    openai: &OpenAI,
    model: Option<String>,
    system_prompt: &str,
    prompt: &str,
    user: Option<String>,
) -> HandlerResult<String> {
    let model = model
        .filter(|m| !m.trim().is_empty())
        .unwrap_or_else(|| CONFIG.DEFAULT_COMPLETIONS_MODEL.to_string());

    tracing::trace!(%model, "enhancing prompt");

    let mut request_builder = CreateChatCompletionRequestArgs::default();
    request_builder.model(&model).messages([
        ChatCompletionRequestSystemMessageArgs::default()
            .content(system_prompt)
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    ]);

    if let Some(user) = user {
        request_builder.user(user);
    }

    let request = request_builder.build()?;
    let response = openai.chat().create(request).await?;

    let enhanced = response
        .choices
        .into_iter()
        .next()
        .and_then(|choice| choice.message.content)
        .map(|content| content.trim().to_string())
        .filter(|content| !content.is_empty())
        .ok_or_else(|| {
            tracing::error!("model returned no enhanced prompt");
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Enhancement failed",
                "The model did not return an enhanced prompt.",
            )
        })?;

    Ok(enhanced)
}
