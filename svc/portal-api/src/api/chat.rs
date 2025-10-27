use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use axum::{Json, extract::State};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::{Deserialize, Serialize};

use crate::{state::AppState, utils::tokens::TokenUsage};

#[derive(Debug, Deserialize)]
pub struct ChatMessageRequest {
    model: Option<String>,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatMessageResponse {
    message: String,
    usage: Option<TokenUsage>,
}

#[axum::debug_handler]
pub async fn handler(
    _: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ChatMessageRequest>,
) -> HandlerResult<Json<ChatMessageResponse>> {
    let client = state.openai.read().await;
    let model = body.model.unwrap_or("@google/gemini-2.5-flash".to_string());
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(2048u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(body.message)
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await.map_err(|e| e)?;
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

    Ok(Json(ChatMessageResponse { message, usage }))
}
