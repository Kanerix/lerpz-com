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
pub(super) async fn write(redis: &RedisPool, id: Uuid, record: &JobRecord) -> Result<(), String> {
    let payload = serde_json::to_string(record).map_err(|err| err.to_string())?;
    let mut conn = redis.get().await.map_err(|err| err.to_string())?;
    redis::cmd("SET")
        .arg(job_key(id))
        .arg(payload)
        .arg("EX")
        .arg(JOB_TTL_SECS)
        .query_async::<()>(&mut *conn)
        .await
        .map_err(|err| err.to_string())
}

/// Read a job record, returning `None` if it does not exist or has expired.
pub(super) async fn read(redis: &RedisPool, id: Uuid) -> Result<Option<JobRecord>, String> {
    let mut conn = redis.get().await.map_err(|err| err.to_string())?;
    let payload: Option<String> = redis::cmd("GET")
        .arg(job_key(id))
        .query_async(&mut *conn)
        .await
        .map_err(|err| err.to_string())?;
    match payload {
        Some(payload) => serde_json::from_str(&payload)
            .map(Some)
            .map_err(|err| err.to_string()),
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
        tracing::error!(%id, "failed to persist job failure: {err}");
    }
}
