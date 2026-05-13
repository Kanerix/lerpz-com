use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use lerpz_axum::problem::{HandlerResult, ProblemSchema};
use serde::Serialize;
use utoipa::ToSchema;

use crate::oapi::HEALTH_TAG;
use crate::state::{AppState, DatabasePool, RedisPool, S3Client};

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthCheck {
    /// Whether the database connection is healthy
    database: bool,
    /// Whether the Redis connection is healthy
    redis: bool,
    /// Whether the S3 connection is healthy
    s3: bool,
}

#[utoipa::path(
    method(get),
    path = "/health",
    operation_id = "health_check",
    tag = HEALTH_TAG,
    summary = "Get API health status",
    description = "Verifies connectivity, always returns 200 if health check succeeds.",
    security(()),
    responses(
        (
            status = OK,
            description = "Service is healthy",
            body = HealthCheck
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
    State(pool): State<DatabasePool>,
    State(redis): State<RedisPool>,
    State(s3): State<S3Client>,
) -> HandlerResult<(StatusCode, Json<HealthCheck>)> {
    let database_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
        .is_ok();

    let mut conn = redis.get().await?;
    let redis_ok = redis::cmd("PING")
        .query_async::<String>(&mut *conn)
        .await
        .is_ok();

    let s3_ok = s3.list_buckets().send().await.is_ok();

    let status_code = if database_ok && redis_ok && s3_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    Ok((
        status_code,
        Json(HealthCheck {
            database: database_ok,
            redis: redis_ok,
            s3: s3_ok,
        }),
    ))
}
