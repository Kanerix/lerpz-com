use axum::{Json, extract::State};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, ProblemSchema},
};

use crate::{
    api::enhance::{EnhanceRequest, EnhanceResponse, enhance_prompt},
    oapi::ENHANCE_TAG,
    state::{AppState, OpenAI},
};

/// Instruction that steers the model to rewrite a chat prompt.
const SYSTEM_PROMPT: &str = "You are a prompt engineer. Rewrite the user's \
    prompt into a clearer, more effective prompt for a conversational AI \
    assistant. Preserve the user's intent, language and any concrete details \
    they provided. Add helpful structure, context and constraints where they \
    are missing, and make the request specific and unambiguous. Do not answer \
    the prompt or add commentary. Respond with the improved prompt only, as \
    plain text.";

#[utoipa::path(
    method(post),
    path = "/chat",
    operation_id = "enhance_chat_prompt",
    tag = ENHANCE_TAG,
    summary = "Enhance a chat prompt",
    description = "Rewrites a raw prompt into a clearer, more effective prompt \
        for a conversational AI assistant.",
    request_body(
        content = EnhanceRequest,
        description = "The prompt to enhance",
        content_type = "application/json"
    ),
    responses(
        (
            status = OK,
            description = "The enhanced prompt",
            body = EnhanceResponse,
            content_type = "application/json"
        ),
        (
            status = BAD_REQUEST,
            description = "Invalid request body",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Unexpected server error",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
    ),
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    token: AzureAccessToken,
    State(openai): State<OpenAI>,
    Json(body): Json<EnhanceRequest>,
) -> HandlerResult<Json<EnhanceResponse>> {
    let prompt =
        enhance_prompt(&openai, body.model, SYSTEM_PROMPT, &body.prompt, token.upn).await?;

    Ok(Json(EnhanceResponse { prompt }))
}
