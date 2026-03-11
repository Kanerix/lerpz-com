use std::convert::Infallible;
use std::sync::Arc;

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
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt as _};

use crate::{
    config::CONFIG,
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    model: Option<String>,
    prompt: String,
    title: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    tag = CHATS_TAG,
    summary = "Create a new chat",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(openai): State<OpenAI>,
    State(database): State<DatabasePool>,
    Json(body): Json<ChatRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let model = body.model.as_deref().unwrap_or(&CONFIG.DEFAULT_TEXT_MODEL);
    let user_id = token.sub;
    let prompt = body.prompt;
    let title = body.title.unwrap_or_else(|| truncate_title(&prompt, 100));

    let conv_id = sqlx::query_scalar!(
        "INSERT INTO conversations (user_id, title, model) VALUES ($1, $2, $3) RETURNING id",
        &user_id,
        &title,
        &model
    )
    .fetch_one(&database)
    .await?;

    sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content) VALUES ($1, 'user', $2)",
        &conv_id,
        &prompt,
    )
    .execute(&database)
    .await?;

    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .max_tokens(2048u32)
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let stream = openai.chat().create_stream(request).await?;

    let assistant_buf = Arc::new(Mutex::new(String::new()));
    let init_event = tokio_stream::once(Ok::<Event, Infallible>(
        Event::default()
            .event("conversation_created")
            .data(conv_id.to_string()),
    ));

    let content_stream = stream.map(move |chunk_result| {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(err) => {
                return Ok(Event::default()
                    .event("error")
                    .data(format!("stream error: {err}")));
            }
        };

        let mut text = String::new();
        let mut is_done = false;

        for choice in &chunk.choices {
            if let Some(ref delta) = choice.delta.content {
                text.push_str(delta);
            }
            if choice.finish_reason.is_some() {
                is_done = true;
            }
        }

        // let text_clone = text.clone();
        // tokio::spawn(async move {
        //     buf.lock().await.push_str(&text_clone);

        //     if is_done {
        //         let full_response = buf.lock().await.clone();
        //         if !full_response.is_empty() {
        //             let result = sqlx::query(
        //                 "INSERT INTO messages (conversation_id, role, content) VALUES ($1, 'assistant', $2)",
        //             )
        //             .bind(conv_id)
        //             .bind(&full_response)
        //             .execute(&db)
        //             .await;

        //             if let Err(err) = result {
        //                 tracing::error!(
        //                     conversation_id = %conv_id,
        //                     "failed to persist assistant message: {err}"
        //                 );
        //             }
        //         }
        //     }
        // });

        if is_done {
            Ok(Event::default().event("done").data(conv_id.to_string()))
        } else {
            Ok(Event::default().event("message").data(text))
        }
    });

    let sse_stream = init_event.chain(content_stream);

    Ok(Sse::new(sse_stream))
}

fn truncate_title(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect::<String>()
}
