use axum::Json;
use axum::extract::State;
use bb8_redis::RedisConnectionManager;
use lerpz_axum::error::HandlerResult;
use serde::Serialize;
use utoipa::ToSchema;

use crate::oapi::HEALTH_TAG;
use crate::state::AppState;

#[derive(Serialize, ToSchema)]
pub struct HealthCheck {
    database: bool,
    redis: bool,
}

#[utoipa::path(
    method(get),
    path = "/health",
    tag = HEALTH_TAG,
    summary = "Get API health status",
    description = "Verifies connectivity, always returns 200 if health check succeeds.",
)]
#[axum::debug_handler(state = AppState)]
pub async fn handler(
    State(pool): State<sqlx::PgPool>,
    State(redis): State<bb8::Pool<RedisConnectionManager>>,
) -> HandlerResult<Json<HealthCheck>> {
    let database_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await
        .is_ok();

    let mut conn = redis.get().await?;
    let redis_ok = redis::cmd("PING")
        .query_async::<String>(&mut *conn)
        .await
        .is_ok();

    Ok(Json(HealthCheck {
        database: database_ok,
        redis: redis_ok,
    }))
}
