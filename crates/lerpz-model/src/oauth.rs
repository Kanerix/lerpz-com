use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Debug, Clone)]
pub struct OAuthClient {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub organization_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct RedirectUri {
    pub id: Uuid,
    pub client_id: Uuid,
    pub uri: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct Scope {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_scope_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct ClientScope {
    pub client_id: Uuid,
    pub scope_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Debug, Clone)]
pub struct RefreshToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub client_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(FromRow, Debug, Clone)]
pub struct AccessToken {
    pub id: Uuid,
    pub jti: String,
    pub user_id: Option<Uuid>,
    pub client_id: Uuid,
    pub scope: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}
