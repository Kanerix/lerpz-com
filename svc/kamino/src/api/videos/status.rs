use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use lerpz_axum::{
    middleware::azure::AzureAccessToken,
    problem::{HandlerResult, Problem, ProblemSchema},
};
use serde::Serialize;
use sqlx::Row as _;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool},
};

use super::list::public_url;

/// The completed video attached to a finished job.
///
/// Mirrors the shape returned by the list endpoint so the frontend can render a
/// job result the same way it renders a listed video.
#[derive(Debug, Serialize, ToSchema)]
pub struct JobVideo {
    /// Unique video ID.
    id: Uuid,
    /// Publicly accessible URL served directly from the storage bucket.
    url: String,
    /// Prompt the video was generated from.
    prompt: String,
    /// Model that generated the video.
    model: String,
    /// Optional AI-generated title.
    title: Option<String>,
    /// Optional AI-generated tags.
    tags: Vec<String>,
    /// Container format (e.g. `mp4`, `webm`).
    format: String,
    /// Video width in pixels.
    width: i32,
    /// Video height in pixels.
    height: i32,
    /// Video duration in seconds.
    duration: i32,
    /// When the video was created.
    created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VideoJobResponse {
    /// The job ID.
    id: Uuid,
    /// Lifecycle status: `in_progress`, `completed`, or `failed`.
    status: String,
    /// Human-readable failure reason. Present only when `status` is `failed`.
    error: Option<String>,
    /// The generated video. Present only when `status` is `completed`.
    video: Option<JobVideo>,
}

#[utoipa::path(
    method(get),
    path = "/jobs/{id}",
    operation_id = "get_video_job",
    tag = VIDEOS_TAG,
    summary = "Get video job status",
    description = "Returns the current status of a video generation job. Poll \
        this endpoint after creating a video until `status` is `completed` \
        (the `video` field carries the result) or `failed` (the `error` field \
        carries the reason).",
    params(
        ("id" = Uuid, Path, description = "The job ID returned by create.")
    ),
    responses(
        (
            status = OK,
            description = "The job's current status",
            body = VideoJobResponse,
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
            description = "No such job for this user",
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
    State(database): State<DatabasePool>,
    Path(id): Path<Uuid>,
) -> HandlerResult<Json<VideoJobResponse>> {
    let oid = token.oid.ok_or(Problem::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unkown token format",
        "Missing Object ID from token",
    ))?;

    // Join the (optional) completed video so a single round-trip returns both
    // the job status and, when finished, the persisted video.
    let row = sqlx::query(
        r#"SELECT j.status AS status, j.error AS error, j.video_id AS video_id,
                  v.prompt AS prompt, v.model AS model, v.title AS title, v.tags AS tags,
                  v.storage_bucket AS storage_bucket, v.storage_key AS storage_key,
                  v.format AS format, v.width AS width, v.height AS height,
                  v.duration AS duration, v.created_at AS created_at
           FROM video_jobs j
           LEFT JOIN video_metadata v ON v.id = j.video_id
           WHERE j.id = $1 AND j.oid = $2"#,
    )
    .bind(id)
    .bind(oid)
    .fetch_optional(&database)
    .await?
    .ok_or(Problem::new(
        StatusCode::NOT_FOUND,
        "Job not found",
        "No video generation job exists with that id.",
    ))?;

    let status: String = row.try_get("status")?;
    let error: Option<String> = row.try_get("error")?;
    let video_id: Option<Uuid> = row.try_get("video_id")?;

    let video = match video_id {
        Some(vid) => {
            let bucket: String = row.try_get("storage_bucket")?;
            let key: String = row.try_get("storage_key")?;
            Some(JobVideo {
                id: vid,
                url: public_url(&bucket, &key),
                prompt: row.try_get("prompt")?,
                model: row.try_get("model")?,
                title: row.try_get("title")?,
                tags: row.try_get::<Option<Vec<String>>, _>("tags")?.unwrap_or_default(),
                format: row.try_get("format")?,
                width: row.try_get("width")?,
                height: row.try_get("height")?,
                duration: row.try_get("duration")?,
                created_at: row.try_get("created_at")?,
            })
        }
        None => None,
    };

    Ok(Json(VideoJobResponse {
        id,
        status,
        error,
        video,
    }))
}
