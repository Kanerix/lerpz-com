use std::convert::Infallible;
use std::sync::Arc;

use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs, FinishReason,
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

    let init_event = tokio_stream::once(Ok::<Event, Infallible>(
        Event::default()
            .event("conversation_created")
            .data(conv_id.to_string()),
    ));

    let assistant_buf = Arc::new(Mutex::new(String::new()));
    let assistant_buf_ref = Arc::clone(&assistant_buf);

    let content_stream = stream.map(move |chunk_result| {
        let chunk = match chunk_result {
            Ok(c) => c,
            Err(err) => {
                return Ok(Event::default()
                    .event("error")
                    .data(format!("stream error: {err}")));
            }
        };

        let mut buf = String::new();
        for choice in &chunk.choices {
            if let Some(ref delta) = choice.delta.content {
                buf.push_str(delta);
            }
            if let Some(finish_reason) = choice.finish_reason {
                match finish_reason {
                    FinishReason::Stop => {
                        let buf_ref_inner = assistant_buf_ref.clone();
                        let buf_clone = buf.clone();
                        let mut guard = buf_ref_inner.blocking_lock();
                        guard.push_str(&buf_clone);

                        return Ok(Event::default().event("done").data(buf));
                    }
                    FinishReason::ContentFilter => {
                        return Ok(Event::default()
                            .event("error")
                            .data("content filter triggered"));
                    }
                    _ => {}
                }
            }
        }

        if !buf.is_empty() {
            let mut guard = assistant_buf_ref.blocking_lock();
            guard.push_str(&buf);
        }

        Ok(Event::default().event("message").data(buf))
    });

    let mut buf_for_save = Some(Arc::clone(&assistant_buf));
    let mut database_for_save = Some(database);
    let save_stream = tokio_stream::once(()).then(move |_| {
        let buf = buf_for_save.take().unwrap();
        let db = database_for_save.take().unwrap();
        async move {
            let content = buf.lock().await.clone();

            let result = sqlx::query!(
                "INSERT INTO messages (conversation_id, role, content) VALUES ($1, 'assistant', $2)",
                &conv_id,
                &content,
            )
            .execute(&db)
            .await;

            match result {
                Ok(_) => Ok(Event::default().event("saved").data(conv_id.to_string())),
                Err(err) => {
                    tracing::error!("failed to save assistant message: {err}");
                    Ok(Event::default()
                        .event("error")
                        .data(format!("failed to save message: {err}")))
                }
            }
        }
    });

    let sse_stream = init_event.chain(content_stream).chain(save_stream);

    Ok(Sse::new(sse_stream))
}

fn truncate_title(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect::<String>()
}
