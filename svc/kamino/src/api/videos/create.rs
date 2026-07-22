use std::convert::Infallible;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{Sse, sse::Event},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
use lerpz_ai::generation::{Family, VertexConfig, VideoEvent, VideoRequest as VideoGenRequest};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use lerpz_metadata::{
    Metadata, MetadataClient,
    models::{GeneralMetadata, GenerationMetadata, StorageMetadata},
};
use serde::Deserialize;
use serde_json::json;
use tokio_stream::{Stream, StreamExt as _};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool, OpenAI, S3Client},
};

#[derive(Debug, Deserialize, ToSchema)]
pub struct VideoRequest {
    /// Prompt that is sent to the model.
    prompt: String,
    /// What model to use.
    ///
    /// This will default to a predefined model if not provided.
    model: Option<String>,
    /// Desired aspect ratio, e.g. `16:9` (landscape) or `9:16` (portrait).
    ///
    /// Defaults to portrait (`9:16`) if not provided or unrecognised.
    aspect_ratio: Option<String>,
    /// Clip length in seconds. Clamped to the range Veo supports (4-8).
    /// Defaults to 8 seconds.
    duration: Option<u16>,
}

/// Build a small `error` SSE event carrying a human-readable message.
///
/// The message is JSON-encoded (a bare string) to match what the frontend
/// unwraps from `error` events.
fn error_event(message: &str) -> Result<Event, Infallible> {
    Ok(Event::default()
        .event("error")
        .json_data(message)
        .expect("failed to serialize error event"))
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_video",
    tag = VIDEOS_TAG,
    summary = "Create a new video",
    request_body(
        content = VideoRequest,
        description = "Video generation parameters",
        content_type = "application/json",
        example = json!({
            "prompt": "A toller dog running along a beach at sunset"
        })
    ),
    responses(
        (
            status = OK,
            description = "SSE stream of video generation events. Events: \
                completed_video ({ b64, format } inline video, or { url } when \
                the provider only exposes a link), \
                saved ({ id } persisted metadata), \
                error (error message)",
            content_type = "text/event-stream"
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
    State(database): State<DatabasePool>,
    State(s3): State<S3Client>,
    Json(body): Json<VideoRequest>,
) -> HandlerResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    let oid = token.oid.ok_or(Problem::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unkown token format",
        "Missing Object ID from token",
    ))?;

    let model_name = body
        .model
        .as_deref()
        .unwrap_or(&CONFIG.DEFAULT_VIDEO_MODEL)
        .to_string();

    // Resolve the model's family so generation can be dispatched to the right
    // provider behaviour. Unknown models fall back to the default family.
    let model_family = sqlx::query_scalar!(
        "SELECT family FROM models WHERE deployment_name = $1 LIMIT 1",
        &model_name,
    )
    .fetch_optional(&database)
    .await?;
    let family = Family::from_name(model_family.as_deref());

    let request = VideoGenRequest {
        prompt: body.prompt.clone(),
        model: model_name.clone(),
        aspect_ratio: body.aspect_ratio.clone(),
        duration: body.duration,
        // The Google/Veo path runs on Vertex AI's native endpoint and needs the
        // project/region and custom host; other families ignore this.
        vertex: matches!(family, Family::Google).then(|| VertexConfig {
            custom_host: CONFIG.VERTEX_BASE_URL.to_string(),
            project_id: CONFIG.VERTEX_PROJECT_ID.to_string(),
            location: CONFIG.VERTEX_LOCATION.to_string(),
        }),
    };

    tracing::debug!(%oid, model = %model_name, "creating video generation job");

    let job = family.start_video(openai.as_ref(), request).await.map_err(|upstream| {
        if upstream.is_user() {
            tracing::warn!(%oid, model = %model_name, reason = %upstream.message, "video generation rejected by provider");
        } else {
            tracing::error!(%oid, model = %model_name, "video generation request failed: {}", upstream.message);
        }
        Problem::new(
            StatusCode::BAD_GATEWAY,
            "Video generation failed",
            upstream.message,
        )
    })?;

    let operation_name = job.operation_name.clone();
    tracing::debug!(%operation_name, model = %model_name, "video generation job created");

    let meta_client = lerpz_metadata::Client::from_pool(database);
    let prompt = body.prompt;

    let sse_stream = async_stream::stream! {
        let mut stream = job.poll();

        // The job yields a single terminal event once rendering finishes.
        if let Some(event) = stream.next().await {
            match event {
                Err(upstream) => {
                    if upstream.is_user() {
                        tracing::warn!(%operation_name, reason = %upstream.message, "video generation rejected by provider");
                    } else {
                        tracing::error!(%operation_name, "video generation failed: {}", upstream.message);
                    }
                    yield error_event(&upstream.message);
                }
                Ok(VideoEvent::Link { url }) => {
                    yield Ok(Event::default()
                        .event("completed_video")
                        .json_data(json!({ "url": url }))
                        .expect("failed to serialize completed_video event"));
                }
                Ok(VideoEvent::Completed { bytes: video_bytes, format, width, height, duration }) => {
                    yield Ok(Event::default()
                        .event("completed_video")
                        .json_data(json!({
                            "b64": BASE64.encode(&video_bytes),
                            "format": format,
                        }))
                        .expect("failed to serialize completed_video event"));

                    let id = Uuid::now_v7();
                    let key = format!("videos/{oid}/{id}.{format}");
                    let now = Utc::now();

                    tracing::trace!(
                        bucket = %CONFIG.AWS_S3_BUCKET,
                        key = %key,
                        "saving video to storage (s3)",
                    );

                    let metadata = Metadata::Video {
                        general: GeneralMetadata { id, created_at: now, updated_at: now },
                        generation: GenerationMetadata {
                            prompt: prompt.clone(),
                            model: model_name.clone(),
                        },
                        storage: StorageMetadata::S3 {
                            bucket: CONFIG.AWS_S3_BUCKET.to_string(),
                            key,
                        },
                        analysis: None,
                        format,
                        width,
                        height,
                        duration,
                    };

                    if let Err(err) = lerpz_metadata::save_to_s3(&s3, &metadata, &video_bytes).await {
                        tracing::error!(%operation_name, "{err}");
                        yield error_event(&err.to_string());
                    } else {
                        tracing::trace!(%operation_name, "persisting metadata to database");
                        match meta_client.insert(metadata).await {
                            Ok(_) => {
                                tracing::trace!(%operation_name, %id, "video generation complete");
                                yield Ok(Event::default()
                                    .event("saved")
                                    .json_data(json!({ "id": id }))
                                    .expect("failed to serialize saved event"));
                            }
                            Err(err) => {
                                tracing::error!(%operation_name, "{err}");
                                yield error_event(&err.to_string());
                            }
                        }
                    }
                }
            }
        }
    };

    Ok(Sse::new(sse_stream))
}
