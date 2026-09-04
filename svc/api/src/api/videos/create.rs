use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
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
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt as _;
use utoipa::ToSchema;
use uuid::Uuid;

use super::job_store::{self, JobRecord};
use crate::{
    config::CONFIG,
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool, OpenAI, RedisPool, S3Client},
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

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateVideoResponse {
    /// Identifier of the created job. Poll `GET /videos/jobs/{id}` with this to
    /// track progress.
    id: Uuid,
    /// Current lifecycle status. Always `in_progress` for a freshly created job.
    status: String,
}

impl IntoResponse for CreateVideoResponse {
    fn into_response(self) -> Response {
        (StatusCode::ACCEPTED, Json(self)).into_response()
    }
}

#[utoipa::path(
    method(post),
    path = "/",
    operation_id = "create_video",
    tag = VIDEOS_TAG,
    summary = "Create a new video",
    description = "Starts a video generation job and returns immediately with a \
        job id. The render runs in the background; poll \
        `GET /videos/jobs/{id}` until the job reaches a terminal state \
        (`completed` or `failed`).",
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
            status = ACCEPTED,
            description = "Job accepted. Poll the returned id for status.",
            body = CreateVideoResponse,
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
            status = BAD_GATEWAY,
            description = "The video provider rejected the request",
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
    State(redis): State<RedisPool>,
    State(s3): State<S3Client>,
    Json(body): Json<VideoRequest>,
) -> HandlerResult<CreateVideoResponse> {
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

    // Kick off the provider render. A failure here means the job could not be
    // created at all, so surface it synchronously rather than via a job record.
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

    // Record the job as in-progress so the client has something to poll while
    // the background task drives the render to completion.
    let job_id = Uuid::now_v7();
    job_store::write(
        &redis,
        job_id,
        &JobRecord {
            oid: oid.clone(),
            status: "in_progress".to_string(),
            error: None,
            video_id: None,
        },
    )
    .await
    .map_err(|err| {
        Problem::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create job",
            err,
        )
    })?;

    tracing::debug!(%job_id, %operation_name, model = %model_name, "video generation job created");

    // Drive the render in the background. The task owns everything it needs, so
    // it outlives the request. Note: an in-flight task is lost on restart, which
    // leaves its record stuck in `in_progress` until the TTL expires it —
    // acceptable for this scope.
    let prompt = body.prompt;
    tokio::spawn(run_job(
        job_id,
        oid,
        prompt,
        model_name,
        operation_name,
        job,
        database,
        redis,
        s3,
    ));

    Ok(CreateVideoResponse {
        id: job_id,
        status: "in_progress".to_string(),
    })
}

/// Runs a video generation job to completion in the background.
///
/// Polls the provider operation, and on success saves the video to storage,
/// persists its metadata, and links it to the job record. Any failure
/// transitions the job to `failed` with a message the client can display.
#[allow(clippy::too_many_arguments)]
async fn run_job(
    job_id: Uuid,
    oid: String,
    prompt: String,
    model_name: String,
    operation_name: String,
    job: lerpz_ai::generation::VideoJob,
    database: DatabasePool,
    redis: RedisPool,
    s3: S3Client,
) {
    let mut stream = job.poll();

    // The job yields a single terminal event once rendering finishes.
    let Some(event) = stream.next().await else {
        tracing::error!(%job_id, %operation_name, "video generation poll ended without an event");
        job_store::fail(&redis, job_id, &oid, "The video generation ended unexpectedly.").await;
        return;
    };

    let (video_bytes, format, width, height, duration) = match event {
        Err(upstream) => {
            if upstream.is_user() {
                tracing::warn!(%operation_name, reason = %upstream.message, "video generation rejected by provider");
            } else {
                tracing::error!(%operation_name, "video generation failed: {}", upstream.message);
            }
            job_store::fail(&redis, job_id, &oid, &upstream.message).await;
            return;
        }
        Ok(VideoEvent::Link { url }) => {
            // We only got a link, not bytes, so we can't persist the video. Fail
            // the job rather than silently dropping the render.
            tracing::error!(%operation_name, %url, "video provider only returned a link; cannot persist");
            job_store::fail(
                &redis,
                job_id,
                &oid,
                "The video could not be downloaded for storage.",
            )
            .await;
            return;
        }
        Ok(VideoEvent::Completed {
            bytes,
            format,
            width,
            height,
            duration,
        }) => (bytes, format, width, height, duration),
    };

    let id = Uuid::now_v7();
    let key = format!("videos/{oid}/{id}.{format}");
    let now = Utc::now();

    tracing::trace!(
        bucket = %CONFIG.AWS_S3_BUCKET,
        key = %key,
        "saving video to storage (s3)",
    );

    let metadata = Metadata::Video {
        general: GeneralMetadata {
            id,
            created_at: now,
            updated_at: now,
        },
        generation: GenerationMetadata {
            prompt,
            model: model_name,
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
        job_store::fail(&redis, job_id, &oid, &err.to_string()).await;
        return;
    }

    tracing::trace!(%operation_name, "persisting metadata to database");
    let meta_client = lerpz_metadata::Client::from_pool(database);
    if let Err(err) = meta_client.insert(metadata).await {
        tracing::error!(%operation_name, "{err}");
        job_store::fail(&redis, job_id, &oid, &err.to_string()).await;
        return;
    }

    if let Err(err) = job_store::write(
        &redis,
        job_id,
        &JobRecord {
            oid,
            status: "completed".to_string(),
            error: None,
            video_id: Some(id),
        },
    )
    .await
    {
        tracing::error!(%job_id, "failed to mark job completed: {err}");
        return;
    }

    tracing::trace!(%operation_name, %job_id, %id, "video generation complete");
}
