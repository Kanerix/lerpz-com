use async_openai::types::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use axum::{Json, extract::State};
use lerpz_axum::error::HandlerResult;
use serde::{Deserialize, Serialize};

use crate::state::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    message: String,
}

#[axum::debug_handler]
pub async fn handler(State(state): State<AppState>) -> HandlerResult<Json<ChatMessage>> {
    let client = state.openai.read().await;

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("@google/gemini-2.5-flash")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;

    let response = client.chat().create(request).await?;

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

    Ok(Json(ChatMessage { message }))
}
