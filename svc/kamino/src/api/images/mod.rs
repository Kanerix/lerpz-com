use async_openai::types::chat::{
    ChatCompletionRequestMessageContentPartImageArgs,
    ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContentPart,
    CreateChatCompletionRequestArgs, ImageUrlArgs,
};
use axum::http::StatusCode;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use lerpz_axum::problem::{HandlerResult, Problem};
use lerpz_metadata::models::AnalysisMetadata;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    config::CONFIG,
    state::{AppState, OpenAI},
};

use utoipa_axum::{router::OpenApiRouter, routes};

mod analysis;
mod analyze_upload;
mod create;
mod delete;
mod edit;
mod list;

/// Instruction that steers the model to describe an image as structured JSON.
const SYSTEM_PROMPT: &str = "You are an assistant that analyses images. Given a \
    single image, produce a concise, descriptive title and a set of relevant \
    tags. Respond with a single JSON object and nothing else, using exactly \
    this shape: {\"title\": string, \"tags\": string[]}. The title should be a \
    short human-readable phrase (no trailing punctuation). Provide between 3 \
    and 10 lowercase tags describing the subject, setting, style, colours and \
    mood. Do not wrap the JSON in markdown fences or add commentary.";

/// Prompt sent alongside the image content part.
const USER_PROMPT: &str = "Analyse this image and return the JSON object.";

/// Title and tags produced by the vision model for an image.
///
/// Shared by both the stored-image analysis endpoint and the
/// "bring your own image" upload endpoint.
#[derive(Debug, Serialize, ToSchema)]
pub struct ImageAnalysisResponse {
    /// AI-generated title describing the image.
    title: String,
    /// AI-generated tags describing the image.
    tags: Vec<String>,
}

/// Shape the model is asked to return.
#[derive(Debug, Deserialize)]
struct AnalysisResult {
    title: String,
    tags: Vec<String>,
}

/// Build a `data:` URL embedding the image so the model can read it inline.
///
/// The image bucket is private (behind authorization), so the model provider
/// cannot fetch a plain URL. Instead the bytes are base64-encoded into a data
/// URL and sent directly in the request.
fn data_url(format: &str, bytes: &[u8]) -> String {
    format!("data:image/{};base64,{}", format, BASE64.encode(bytes))
}

/// Extract the first balanced JSON object from a model reply.
///
/// Vision models occasionally wrap their answer in prose or markdown fences
/// despite instructions, so trim to the outermost `{...}` before parsing.
fn extract_json(content: &str) -> Option<&str> {
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    (start <= end).then(|| &content[start..=end])
}

/// Run the vision model over raw image bytes and return the parsed analysis.
///
/// `format` is the image subtype (e.g. `png`, `jpeg`) used to build the inline
/// data URL. `user` is an optional stable user identifier forwarded to the
/// model provider for abuse monitoring.
async fn analyze_bytes(
    openai: &OpenAI,
    format: &str,
    bytes: &[u8],
    user: Option<String>,
) -> HandlerResult<AnalysisMetadata> {
    let url = data_url(format, bytes);

    let mut request_builder = CreateChatCompletionRequestArgs::default();
    request_builder
        .model(CONFIG.DEFAULT_COMPLETIONS_MODEL.as_ref())
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(SYSTEM_PROMPT)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(vec![
                    ChatCompletionRequestUserMessageContentPart::Text(
                        ChatCompletionRequestMessageContentPartTextArgs::default()
                            .text(USER_PROMPT)
                            .build()?,
                    ),
                    ChatCompletionRequestUserMessageContentPart::ImageUrl(
                        ChatCompletionRequestMessageContentPartImageArgs::default()
                            .image_url(ImageUrlArgs::default().url(url).build()?)
                            .build()?,
                    ),
                ])
                .build()?
                .into(),
        ]);

    if let Some(user) = user {
        request_builder.user(user);
    }

    let request = request_builder.build()?;

    tracing::trace!("requesting image analysis from model");
    let response = openai.chat().create(request).await.map_err(|err| {
        Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Analysis failed",
            "The analysis request to the model failed.",
        )
        .with_error(err)
    })?;

    let content = response
        .choices
        .into_iter()
        .next()
        .and_then(|choice| choice.message.content)
        .ok_or_else(|| {
            tracing::error!("model returned no analysis");
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Analysis failed",
                "The model did not return an analysis.",
            )
        })?;

    let result: AnalysisResult = extract_json(&content)
        .and_then(|json| serde_json::from_str(json).ok())
        .ok_or_else(|| {
            tracing::error!(%content, "failed to parse analysis JSON");
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Analysis failed",
                "The model did not return a valid analysis.",
            )
        })?;

    Ok(AnalysisMetadata {
        title: result.title.trim().to_string(),
        tags: result.tags,
    })
}

pub fn router() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create::handler, list::handler))
        .routes(routes!(edit::handler))
        .routes(routes!(delete::handler))
        .routes(routes!(analysis::handler))
        .routes(routes!(analyze_upload::handler))
}
