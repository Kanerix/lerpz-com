use axum::extract::FromRef;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub database: sqlx::PgPool,
    pub redis: bb8::Pool<bb8_redis::RedisConnectionManager>,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Pool<Postgres> {
        state.database.clone()
    }
}

impl FromRef<AppState> for bb8::Pool<bb8_redis::RedisConnectionManager> {
    fn from_ref(state: &AppState) -> Self {
        state.redis.clone()
    }
}
