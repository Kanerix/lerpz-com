use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
