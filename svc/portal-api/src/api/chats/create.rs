use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use axum::{Json, extract::State};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::{Deserialize, Serialize};

use crate::{config::CONFIG, state::AppState, utils::tokens::TokenUsage};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    model: Option<String>,
    prompt: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    message: String,
    usage: Option<TokenUsage>,
}

#[axum::debug_handler]
pub async fn handler(
    _: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> HandlerResult<Json<ChatResponse>> {
    let model = body.model.as_deref().unwrap_or(&CONFIG.DEFAULT_TEXT_MODEL);
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(2048u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(body.prompt)
            .build()?
            .into()])
        .build()?;

    let client = state.openai.read().await;
    let response = client.chat().create(request).await?;

    let usage = response.usage.map(|a| a.into());
    let choice = response
        .choices
        .iter()
        .next()
        .ok_or(anyhow::anyhow!("Model did not generate a choice"))?;

    let message = choice
        .message
        .content
        .clone()
        .ok_or(anyhow::anyhow!("Model did not generate a message"))?;

    Ok(Json(ChatResponse { message, usage }))
}
