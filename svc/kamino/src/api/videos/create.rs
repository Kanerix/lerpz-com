use std::{convert::Infallible, time::Duration};

use async_openai::types::videos::{CreateVideoRequestArgs, VideoSeconds, VideoSize, VideoStatus};
use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{Sse, sse::Event},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use chrono::Utc;
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
use tokio_stream::Stream;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    config::CONFIG,
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool, OpenAI, S3Client},
};

/// How long to wait between polls of the generation job.
const POLL_INTERVAL: Duration = Duration::from_secs(5);
/// Container format Sora renders to.
const VIDEO_FORMAT: &str = "mp4";

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
    /// Clip length in seconds. Rounded down to the nearest supported value
    /// (4, 8 or 12). Defaults to 4 seconds.
    duration: Option<u16>,
}

/// Map a requested aspect ratio to a supported output resolution.
fn resolve_size(aspect_ratio: Option<&str>) -> VideoSize {
    match aspect_ratio.map(str::trim) {
        Some("16:9") | Some("landscape") => VideoSize::S1280x720,
        _ => VideoSize::S720x1280,
    }
}

/// Map a requested duration to a supported clip length.
fn resolve_seconds(duration: Option<u16>) -> VideoSeconds {
    match duration.unwrap_or(4) {
        0..=5 => VideoSeconds::Four,
        6..=10 => VideoSeconds::Eight,
        _ => VideoSeconds::Twelve,
    }
}

/// Pixel dimensions for a given output resolution.
fn dimensions(size: &VideoSize) -> (u32, u32) {
    match size {
        VideoSize::S720x1280 => (720, 1280),
        VideoSize::S1280x720 => (1280, 720),
        VideoSize::S1024x1792 => (1024, 1792),
        VideoSize::S1792x1024 => (1792, 1024),
    }
}

/// Clip length in seconds as a number.
fn seconds_value(seconds: &VideoSeconds) -> u32 {
    match seconds {
        VideoSeconds::Four => 4,
        VideoSeconds::Eight => 8,
        VideoSeconds::Twelve => 12,
    }
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
                partial_video ({ progress } completion percentage), \
                completed_video ({ b64, format } final video), \
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

    let size = resolve_size(body.aspect_ratio.as_deref());
    let seconds = resolve_seconds(body.duration);

    let mut request_builder = CreateVideoRequestArgs::default();
    request_builder
        .model(model_name.clone())
        .prompt(body.prompt.clone())
        .size(size)
        .seconds(seconds);

    let request = request_builder.build()?;

    tracing::debug!(
        %oid,
        model = %model_name,
        provider = %CONFIG.PORTKEY_PROVIDER,
        base_url = %CONFIG.PORTKEY_BASE_URL,
        "creating video generation job",
    );

    // Sora generation is asynchronous: creating the job returns immediately
    // with a handle we then poll until it completes or fails.
    //
    // Portkey passes upstream provider errors straight through, often in a
    // non-OpenAI shape (e.g. `{"status":"failure","message":"…"}`), which
    // `async-openai` can't deserialize into its error type. Surface the
    // humanized upstream message instead of letting it collapse into an
    // opaque 500.
    let job = openai.videos().create(request).await.map_err(|err| {
        tracing::error!(
            %oid,
            model = %model_name,
            provider = %CONFIG.PORTKEY_PROVIDER,
            base_url = %CONFIG.PORTKEY_BASE_URL,
            "failed to create video job: {err}",
        );
        Problem::new(
            StatusCode::BAD_GATEWAY,
            "Video generation failed",
            lerpz_portkey::humanize_error(&err.to_string()),
        )
    })?;
    let job_id = job.id.clone();

    tracing::debug!(%job_id, model = %model_name, "video generation job created");

    let meta_client = lerpz_metadata::Client::from_pool(database);

    let sse_stream = async_stream::stream! {
        tracing::trace!(%job_id, "starting video generation poll");
        let mut last_progress = job.progress;

        loop {
            tokio::time::sleep(POLL_INTERVAL).await;

            let current = match openai.videos().retrieve(&job_id).await {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!(%job_id, "failed to poll video job: {err}");
                    yield Ok(Event::default()
                        .event("error")
                        .json_data(lerpz_portkey::humanize_error(&err.to_string()))
                        .expect("failed to serialize error event"));
                    break;
                }
            };

            match current.status {
                VideoStatus::Queued | VideoStatus::InProgress => {
                    if current.progress != last_progress {
                        last_progress = current.progress;
                        tracing::trace!(%job_id, progress = %current.progress, "video generation progress");
                        yield Ok(Event::default()
                            .event("partial_video")
                            .json_data(json!({ "progress": current.progress }))
                            .expect("failed to serialize partial_video event"));
                    }
                }
                VideoStatus::Failed => {
                    let message = current
                        .error
                        .map(|e| e.message)
                        .unwrap_or_else(|| "Video generation failed.".to_string());
                    tracing::error!(%job_id, %message, "video generation failed");
                    yield Ok(Event::default()
                        .event("error")
                        .json_data(lerpz_portkey::humanize_error(&message))
                        .expect("failed to serialize error event"));
                    break;
                }
                VideoStatus::Completed => {
                    tracing::trace!(%job_id, "downloading completed video");
                    let video_bytes = match openai.videos().download_content(&job_id).await {
                        Ok(bytes) => bytes.to_vec(),
                        Err(err) => {
                            tracing::error!(%job_id, "failed to download video: {err}");
                            yield Ok(Event::default()
                                .event("error")
                                .json_data(lerpz_portkey::humanize_error(&err.to_string()))
                                .expect("failed to serialize error event"));
                            break;
                        }
                    };
                    tracing::trace!(%job_id, bytes = video_bytes.len(), "downloaded completed video");

                    yield Ok(Event::default()
                        .event("completed_video")
                        .json_data(json!({
                            "b64": BASE64.encode(&video_bytes),
                            "format": VIDEO_FORMAT,
                        }))
                        .expect("failed to serialize completed_video event"));

                    let (width, height) = dimensions(&current.size);
                    let duration = seconds_value(&current.seconds);

                    let id = Uuid::now_v7();
                    let key = format!("videos/{oid}/{id}.{VIDEO_FORMAT}");
                    let now = Utc::now();

                    tracing::trace!(
                        bucket = %CONFIG.AWS_S3_BUCKET,
                        key = %key,
                        "saving video to storage (s3)",
                    );

                    let metadata = Metadata::Video {
                        general: GeneralMetadata { id, created_at: now, updated_at: now },
                        generation: GenerationMetadata {
                            prompt: body.prompt.clone(),
                            model: model_name.clone(),
                        },
                        storage: StorageMetadata::S3 {
                            bucket: CONFIG.AWS_S3_BUCKET.to_string(),
                            key,
                        },
                        analysis: None,
                        format: VIDEO_FORMAT.to_string(),
                        width,
                        height,
                        duration,
                    };

                    if let Err(err) = lerpz_metadata::save_to_s3(&s3, &metadata, &video_bytes).await {
                        tracing::error!(%job_id, "{err}");
                        yield Ok(Event::default()
                            .event("error")
                            .json_data(err.to_string())
                            .expect("failed to serialize error event"));
                        break;
                    }

                    tracing::trace!(%job_id, "persisting metadata to database");
                    match meta_client.insert(metadata).await {
                        Ok(_) => {
                            tracing::trace!(%job_id, %id, "video generation complete");
                            yield Ok(Event::default()
                                .event("saved")
                                .json_data(json!({ "id": id }))
                                .expect("failed to serialize saved event"));
                        }
                        Err(err) => {
                            tracing::error!(%job_id, "{err}");
                            yield Ok(Event::default()
                                .event("error")
                                .json_data(err.to_string())
                                .expect("failed to serialize error event"));
                        }
                    }

                    break;
                }
            }
        }
    };

    Ok(Sse::new(sse_stream))
}
