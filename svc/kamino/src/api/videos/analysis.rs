use async_openai::types::chat::{
    ChatCompletionRequestMessageContentPartImageArgs,
    ChatCompletionRequestMessageContentPartTextArgs, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs, ChatCompletionRequestUserMessageContentPart,
    CreateChatCompletionRequestArgs, ImageUrlArgs,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use lerpz_metadata::{
    Metadata, MetadataClient, MetadataKind,
    models::{AnalysisMetadata, StorageMetadata},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool, OpenAI, S3Client},
};

/// Instruction that steers the model to describe a video as structured JSON.
const SYSTEM_PROMPT: &str = "You are an assistant that analyses videos. Given a \
    single video, produce a concise, descriptive title and a set of relevant \
    tags. Respond with a single JSON object and nothing else, using exactly \
    this shape: {\"title\": string, \"tags\": string[]}. The title should be a \
    short human-readable phrase (no trailing punctuation). Provide between 3 \
    and 10 lowercase tags describing the subject, setting, style, colours, \
    motion and mood. Do not wrap the JSON in markdown fences or add commentary.";

/// Prompt sent alongside the video content part.
const USER_PROMPT: &str = "Analyse this video and return the JSON object.";

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoAnalysisResponse {
    /// AI-generated title describing the video.
    title: String,
    /// AI-generated tags describing the video.
    tags: Vec<String>,
}

/// Shape the model is asked to return.
#[derive(Debug, Deserialize)]
struct AnalysisResult {
    title: String,
    tags: Vec<String>,
}

/// Build a `data:` URL embedding the video so the model can read it inline.
///
/// The video bucket is private (behind authorization), so the model provider
/// cannot fetch a plain URL. Instead the bytes are base64-encoded into a data
/// URL and sent directly in the request.
fn data_url(format: &str, bytes: &[u8]) -> String {
    format!("data:video/{};base64,{}", format, BASE64.encode(bytes))
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

#[utoipa::path(
    method(post),
    path = "/{id}/analysis",
    operation_id = "analyze_video",
    tag = VIDEOS_TAG,
    summary = "Analyse a video",
    description = "Runs a vision model over a previously generated video to \
        produce a descriptive title and a set of tags, persists them to the \
        video's metadata, and returns them. Re-running the analysis overwrites \
        any existing title and tags.",
    params(
        ("id" = Uuid, Path, description = "Video ID"),
    ),
    responses(
        (
            status = OK,
            description = "The generated analysis",
            body = VideoAnalysisResponse,
            content_type = "application/json"
        ),
        (
            status = UNAUTHORIZED,
            description = "Missing or invalid authentication token",
            body = ProblemSchema,
            content_type = "application/problem+json"
        ),
        (
            status = NOT_FOUND,
            description = "Video not found",
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
    State(database): State<DatabasePool>,
    State(s3): State<S3Client>,
    Path(id): Path<Uuid>,
) -> HandlerResult<Json<VideoAnalysisResponse>> {
    let meta_client = lerpz_metadata::Client::from_pool(database);

    tracing::trace!(%id, "fetching video metadata from database");
    let metadata = meta_client
        .get(id, MetadataKind::Video)
        .await
        .map_err(|err| match err {
            lerpz_metadata::error::Error::NotFound(_) => Problem::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                "The requested video was not found.",
            ),
            err => {
                tracing::error!(%id, error = ?err, "failed to fetch video metadata");
                Problem::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    "Failed to fetch video metadata.",
                )
                .with_error(err)
            }
        })?;

    let Metadata::Video {
        general,
        generation,
        storage,
        format,
        width,
        height,
        duration,
        ..
    } = metadata
    else {
        tracing::error!(%id, "unexpected metadata kind returned for video");
        return Err(Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Unexpected metadata kind returned for video.",
        ));
    };

    // The video bucket is private, so the model provider cannot fetch it by
    // URL. Download the bytes ourselves (we hold the credentials) and inline
    // them as a base64 data URL.
    let (bucket, key) = match &storage {
        StorageMetadata::S3 { bucket, key } => (bucket, key),
        StorageMetadata::ABS { .. } => {
            tracing::error!(%id, "unsupported storage backend for video analysis");
            return Err(Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Unsupported storage backend for video analysis.",
            ));
        }
    };

    tracing::trace!(%id, %bucket, %key, "fetching video bytes from storage");
    let object = s3
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .map_err(|err| {
            tracing::error!(%id, %bucket, %key, error = ?err, "failed to fetch video from storage");
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Failed to fetch video from storage.",
            )
            .with_error(err)
        })?;

    let bytes = object.body.collect().await.map_err(|err| {
        tracing::error!(%id, %bucket, %key, error = ?err, "failed to read video bytes from storage");
        Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            "Failed to read video from storage.",
        )
        .with_error(err)
    })?;

    let url = data_url(&format, &bytes.into_bytes());

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

    if let Some(upn) = token.upn {
        request_builder.user(upn);
    }

    let request = request_builder.build()?;

    tracing::trace!(%id, "requesting video analysis from model");
    let response = openai.chat().create(request).await.map_err(|err| {
        tracing::error!(%id, error = ?err, "video analysis model request failed");
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

    let analysis = AnalysisMetadata {
        title: result.title.trim().to_string(),
        tags: result.tags,
    };
    let response = VideoAnalysisResponse {
        title: analysis.title.clone(),
        tags: analysis.tags.clone(),
    };

    tracing::trace!(%id, "persisting video analysis to database");
    meta_client
        .update(
            id,
            Metadata::Video {
                general,
                generation,
                storage,
                analysis: Some(analysis),
                format,
                width,
                height,
                duration,
            },
        )
        .await
        .map_err(|err| {
            tracing::error!(%id, error = ?err, "failed to persist video analysis");
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                "Failed to persist video analysis.",
            )
            .with_error(err)
        })?;

    Ok(Json(response))
}
