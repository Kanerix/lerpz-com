use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs, FinishReason,
};
use axum::http::StatusCode;
use axum::{
    Json,
    extract::{Path, State},
    response::{Sse, sse::Event},
};
use lerpz_axum::{
    error::{HandlerError, HandlerResult},
    middleware::azure::AzureAccessToken,
};
use serde::Deserialize;
use tokio_stream::{Stream, StreamExt as _};
use uuid::Uuid;

use crate::{
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize)]
pub struct MessageRequest {
    prompt: String,
}

#[utoipa::path(
    method(post),
    path = "/{id}",
    tag = CHATS_TAG,
    summary = "Send a message to an existing chat",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    Path(conv_id): Path<Uuid>,
    State(openai): State<OpenAI>,
    State(database): State<DatabasePool>,
    Json(body): Json<MessageRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let user_id = token.sub;
    let prompt = body.prompt;

    let conversation = sqlx::query!(
        "SELECT id, model FROM conversations WHERE id = $1 AND user_id = $2",
        &conv_id,
        &user_id,
    )
    .fetch_optional(&database)
    .await?;

    let conversation = match conversation {
        Some(c) => c,
        None => {
            return Err(HandlerError::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested conversation was not found.",
            ));
        }
    };

    let model = conversation.model;

    let previous_messages = sqlx::query!(
        "SELECT role AS \"role: String\", content
        FROM messages
        WHERE conversation_id = $1
        ORDER BY created_at ASC",
        &conv_id,
    )
    .fetch_all(&database)
    .await?;

    sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content)
        VALUES ($1, 'user', $2)",
        &conv_id,
        &prompt,
    )
    .execute(&database)
    .await?;

    let mut messages: Vec<async_openai::types::chat::ChatCompletionRequestMessage> = Vec::new();

    for msg in &previous_messages {
        match msg.role.as_str() {
            "user" => {
                messages.push(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(msg.content.clone())
                        .build()?
                        .into(),
                );
            }
            "assistant" => {
                messages.push(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(msg.content.clone())
                        .build()?
                        .into(),
                );
            }
            _ => {}
        }
    }

    messages.push(
        ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into(),
    );

    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .max_tokens(2048u32)
        .model(&model)
        .messages(messages)
        .stream(true);

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let mut stream = openai.chat().create_stream(request).await?;

    let sse_stream = async_stream::stream! {
        let mut assistant_buf = String::new();
        let mut completed = false;

        while let Some(chunk_result) = stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(err) => {
                    yield Ok(Event::default()
                        .event("error")
                        .data(format!("stream error: {err}")));
                    continue;
                }
            };

            let mut buf = String::new();
            let mut finished = false;

            for choice in &chunk.choices {
                if let Some(ref delta) = choice.delta.content {
                    buf.push_str(delta);
                }
                if let Some(finish_reason) = choice.finish_reason {
                    match finish_reason {
                        FinishReason::Stop => {
                            completed = true;
                            finished = true;
                        }
                        FinishReason::ContentFilter => {
                            yield Ok(Event::default()
                                .event("error")
                                .data("content filter triggered"));
                            continue;
                        }
                        _ => {}
                    }
                }
            }

            assistant_buf.push_str(&buf);

            if finished {
                yield Ok(Event::default().event("done").data(buf));
            } else {
                yield Ok(Event::default().event("message").data(buf));
            }
        }

        if !completed {
            yield Ok(Event::default()
                .event("error")
                .data("stream ended without completion"));
            return;
        }

        let result = sqlx::query!(
            "INSERT INTO messages (conversation_id, role, content) VALUES ($1, 'assistant', $2)",
            &conv_id,
            &assistant_buf,
        )
        .execute(&database)
        .await;

        match result {
            Ok(_) => yield Ok(Event::default().event("saved").data(conv_id.to_string())),
            Err(err) => {
                tracing::error!("failed to save assistant message: {err}");
                yield Ok(Event::default()
                    .event("error")
                    .data(format!("failed to save message: {err}")));
            }
        }
    };

    Ok(Sse::new(sse_stream))
}
