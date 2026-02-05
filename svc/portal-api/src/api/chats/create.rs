use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use axum::{
    Json,
    extract::State,
    response::{Sse, sse::Event},
};
use lerpz_axum::{error::HandlerResult, middleware::azure::AzureAccessToken};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};

use crate::{config::CONFIG, oapi::CHATS_TAG, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    model: Option<String>,
    prompt: String,
}

#[utoipa::path(
    method(post),
    path = "/",
    tag = CHATS_TAG,
    summary = "Create a new chat",
)]
#[axum::debug_handler]
pub async fn handler(
    token: AzureAccessToken,
    State(state): State<AppState>,
    Json(body): Json<ChatRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let model = body.model.as_deref().unwrap_or(&CONFIG.DEFAULT_TEXT_MODEL);
    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .max_tokens(2048u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(body.prompt)
            .build()?
            .into()])
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let client = state.openai.read().await;
    let stream = client.chat().create_stream(request).await?;

    let sse_stream = stream.map(|chunk_result| {
        // If the upstream stream errors, turn it into an SSE "error" event
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(err) => {
                let event = Event::default()
                    .event("error")
                    .data(format!("stream error: {err}"));
                return Ok(event);
            }
        };

        // Extract text from the delta(s)
        // Each chunk may contain multiple choices; we concatenate their deltas.
        let mut buf = String::new();
        for choice in chunk.choices {
            if let Some(delta) = choice.delta.content {
                // `delta` is typically Vec<ChatCompletionMessageToolChoice> or similar;
                // the exact shape depends on your async-openai version. If it's just a String, use it directly.
                buf.push_str(&delta);
            }
        }

        // Build an SSE event
        let event = Event::default()
            .event("message") // optional: "message" event type for the client
            .data(buf);

        Ok(event)
    });

    Ok(Sse::new(sse_stream))
}
