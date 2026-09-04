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

use super::job_store;
use super::list::public_url;
use crate::{
    oapi::VIDEOS_TAG,
    state::{AppState, DatabasePool, RedisPool},
};

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
            description = "No such job for this user, or it has expired",
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
    State(redis): State<RedisPool>,
    Path(id): Path<Uuid>,
) -> HandlerResult<Json<VideoJobResponse>> {
    let oid = token.oid.ok_or(Problem::new(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unkown token format",
        "Missing Object ID from token",
    ))?;

    let not_found = || {
        Problem::new(
            StatusCode::NOT_FOUND,
            "Job not found",
            "No video generation job exists with that id, or it has expired.",
        )
    };

    let record = job_store::read(&redis, id)
        .await
        .map_err(|err| {
            Problem::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to read job",
                err,
            )
        })?
        .ok_or_else(not_found)?;

    // Don't leak another user's jobs; an ownership mismatch is a "not found".
    if record.oid != oid {
        return Err(not_found());
    }

    // On completion the durable record lives in `video_metadata`; load it by id
    // to build the result the same way the list endpoint does.
    let video = match record.video_id {
        Some(vid) => {
            let row = sqlx::query(
                r#"SELECT prompt, model, title, tags, storage_bucket, storage_key,
                          format, width, height, duration, created_at
                   FROM video_metadata
                   WHERE id = $1"#,
            )
            .bind(vid)
            .fetch_optional(&database)
            .await?;

            match row {
                Some(row) => {
                    let bucket: String = row.try_get("storage_bucket")?;
                    let key: String = row.try_get("storage_key")?;
                    Some(JobVideo {
                        id: vid,
                        url: public_url(&bucket, &key),
                        prompt: row.try_get("prompt")?,
                        model: row.try_get("model")?,
                        title: row.try_get("title")?,
                        tags: row
                            .try_get::<Option<Vec<String>>, _>("tags")?
                            .unwrap_or_default(),
                        format: row.try_get("format")?,
                        width: row.try_get("width")?,
                        height: row.try_get("height")?,
                        duration: row.try_get("duration")?,
                        created_at: row.try_get("created_at")?,
                    })
                }
                None => None,
            }
        }
        None => None,
    };

    Ok(Json(VideoJobResponse {
        id,
        status: record.status,
        error: record.error,
        video,
    }))
}
