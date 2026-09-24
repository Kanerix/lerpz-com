//! Redis-backed store for transient video generation job state.
//!
//! A video render is a long-running provider operation driven by a background
//! task. A job record is ephemeral status the client polls until the render
//! reaches a terminal state; the durable artefacts are the `video_metadata`
//! row and the stored object. Records carry a TTL so completed and abandoned
//! jobs expire on their own without a cleanup sweep.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::RedisPool;

/// A type alias for handling results from this module.
pub(super) type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while reading or writing a job record.
///
/// The messages are deliberately coarse. They name the step that failed and
/// leave the detail to the source error, which is only ever logged.
#[derive(Debug, thiserror::Error)]
pub(super) enum Error {
    #[error("no redis connection available")]
    Connection(#[from] bb8::RunError<redis::RedisError>),
    #[error("redis command failed")]
    Command(#[from] redis::RedisError),
    #[error("job record is not valid json")]
    Payload(#[from] serde_json::Error),
}

/// How long (in seconds) a job record lives before Redis expires it. Long
/// enough for a client to finish polling a completed render, short enough that
/// dead jobs don't linger.
const JOB_TTL_SECS: i64 = 3600;

/// Ephemeral lifecycle state of a video generation job.
#[derive(Debug, Serialize, Deserialize)]
pub(super) struct JobRecord {
    /// Azure object id of the user who created the job, used for ownership
    /// checks when the status is read back.
    pub oid: String,
    /// One of `in_progress`, `completed`, or `failed`.
    pub status: String,
    /// Human-readable failure reason. Set only when `status` is `failed`.
    pub error: Option<String>,
    /// The persisted video's id. Set only when `status` is `completed`.
    pub video_id: Option<Uuid>,
}

/// Redis key for a job's record.
fn job_key(id: Uuid) -> String {
    format!("video:job:{id}")
}

/// Write (or overwrite) a job record, refreshing its TTL.
pub(super) async fn write(redis: &RedisPool, id: Uuid, record: &JobRecord) -> Result<()> {
    let payload = serde_json::to_string(record)?;
    let mut conn = redis.get().await?;
    redis::cmd("SET")
        .arg(job_key(id))
        .arg(payload)
        .arg("EX")
        .arg(JOB_TTL_SECS)
        .query_async::<()>(&mut *conn)
        .await?;
    Ok(())
}

/// Read a job record, returning `None` if it does not exist or has expired.
pub(super) async fn read(redis: &RedisPool, id: Uuid) -> Result<Option<JobRecord>> {
    let mut conn = redis.get().await?;
    let payload: Option<String> = redis::cmd("GET")
        .arg(job_key(id))
        .query_async(&mut *conn)
        .await?;
    match payload {
        Some(payload) => Ok(Some(serde_json::from_str(&payload)?)),
        None => Ok(None),
    }
}

/// Transition a job to `failed` with a human-readable reason.
///
/// Best-effort: a failure to persist is logged rather than propagated, since
/// this runs in a background task with nowhere to surface the error.
pub(super) async fn fail(redis: &RedisPool, id: Uuid, oid: &str, message: &str) {
    let record = JobRecord {
        oid: oid.to_string(),
        status: "failed".to_string(),
        error: Some(message.to_string()),
        video_id: None,
    };
    if let Err(err) = write(redis, id, &record).await {
        tracing::error!(%id, ?err, "failed to persist job failure");
    }
}
