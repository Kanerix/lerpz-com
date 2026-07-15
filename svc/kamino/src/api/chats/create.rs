use std::convert::Infallible;

use async_openai::types::chat::{
    ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
};
use axum::{
    Json,
    extract::State,
    response::{Sse, sse::Event},
};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};
use serde::Deserialize;
use tokio_stream::Stream;
use utoipa::ToSchema;

use crate::{
    api::chats::stream::start_completion_sse,
    config::CONFIG,
    oapi::CHATS_TAG,
    state::{AppState, DatabasePool, OpenAI},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct ChatRequest {
    /// Optional model override (uses the configured default when omitted)
    model: Option<String>,
    /// The user's first message / opening prompt
    prompt: String,
    /// Optional conversation title; auto-generated from the prompt when omitted
    title: Option<String>,
    /// Reasoning level for reasoning-capable models (`none`, `minimal`, `low`,
    /// `medium`, `high` or `xhigh`). Unknown values fall back to `low`. Uses
    /// the model's default behaviour when omitted.
    reasoning: Option<String>,
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_chat",
    tag = CHATS_TAG,
    summary = "Create a new chat",
    description = "Creates a new conversation, streams the AI reply back via Server-Sent Events. \
        The first event is `conversation_created` with the new conversation ID; \
        subsequent `reasoning` events carry incremental chain-of-thought chunks \
        (reasoning models only) and `message` events carry incremental answer \
        token chunks until the stream ends; \
        `saved` confirms the reply was persisted and is the final event before the stream closes; \
        `error` is emitted on failures.",
    request_body(
        content = ChatRequest,
        description = "Chat creation parameters",
        content_type = "application/json",
    ),
    responses(
        (
            status = OK,
            description = "SSE stream of AI response chunks. Events: \
                conversation_created (conversation UUID), \
                reasoning (reasoning token chunk), \
                message (token chunk), \
                saved (conversation UUID), \
                error (error message)",
            content_type = "text/event-stream",
            body = String,
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json",
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(openai): State<OpenAI>,
    State(database): State<DatabasePool>,
    Json(body): Json<ChatRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let model = body
        .model
        .as_deref()
        .unwrap_or(&CONFIG.DEFAULT_COMPLETIONS_MODEL);
    let user_id = token.sub;
    let prompt = body.prompt;
    let reasoning = body.reasoning;
    let title = body.title.unwrap_or_else(|| truncate_title(&prompt, 100));

    tracing::trace!(%user_id, %model, "creating conversation");
    let conv_id = sqlx::query_scalar!(
        "INSERT INTO conversations (user_id, title, model)
        VALUES ($1, $2, $3)
        RETURNING id",
        &user_id,
        &title,
        &model
    )
    .fetch_one(&database)
    .await?;

    tracing::trace!(%conv_id, "persisting user message");
    sqlx::query!(
        "INSERT INTO messages (conversation_id, role, content)
        VALUES ($1, 'user', $2)",
        &conv_id,
        &prompt,
    )
    .execute(&database)
    .await?;

    // Resolve the model's family so assistant replies can be tagged with the
    // provider that generated them. Unknown models (e.g. a raw default that
    // isn't registered) simply leave the family unset.
    let model_family = sqlx::query_scalar!(
        "SELECT family FROM models WHERE deployment_name = $1 LIMIT 1",
        model,
    )
    .fetch_optional(&database)
    .await?;

    let mut request_builder = CreateChatCompletionRequestArgs::default();

    request_builder
        .model(model)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .stream(true);

    if let Some(level) = reasoning.as_deref() {
        let reasoning_level = super::parse_reasoning_effort(level);
        request_builder.reasoning_effort(reasoning_level);
    }

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;
    let reply_stream =
        start_completion_sse(openai, request, conv_id, database, model_family).await?;

    let sse_stream = async_stream::stream! {
        yield Ok(Event::default()
            .event("conversation_created")
            .data(conv_id.to_string()));

        for await event in reply_stream {
            yield event;
        }
    };

    Ok(Sse::new(sse_stream))
}

#[inline]
fn truncate_title(s: &str, max_len: usize) -> String {
    s.chars().take(max_len).collect::<String>()
}
