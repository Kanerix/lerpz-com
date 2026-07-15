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

/// Instruction that steers the model to rewrite a video-generation prompt.
const SYSTEM_PROMPT: &str = "You are a prompt engineer for text-to-video \
    generation models. Rewrite the user's prompt into a vivid, cinematic video \
    prompt. Preserve the user's intent and any concrete details they provided, \
    then enrich it with subject and action, camera movement and shot type, \
    setting, lighting, mood, pacing and visual style where they are missing. \
    Describe motion and how the scene evolves over time. Keep it a single \
    coherent description rather than a list of questions. Do not add commentary \
    or explanations. Respond with the improved prompt only, as plain text.";

#[utoipa::path(
    method(post),
    path = "/video",
    operation_id = "enhance_video_prompt",
    tag = ENHANCE_TAG,
    summary = "Enhance a video prompt",
    description = "Rewrites a raw prompt into a vivid, cinematic prompt for a \
        text-to-video generation model.",
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
