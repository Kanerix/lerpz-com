//! Claims for a JWT token.

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represent all claims for a token.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    /// What audience the token is for.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub aud: String,
    /// Who issued the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub iss: String,
    /// Subject of the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sub: String,
    /// Unique identifier for the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub jti: String,
    /// Which time the token will expire.
    pub exp: i64,
    /// When the token will be valid.
    pub nbf: i64,
    /// When the token was issued.
    pub iat: i64,
    /// Scope of the token.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub scp: String
}

impl Default for Claims {
    fn default() -> Self {
        Claims {
            aud: String::new(),
            iss: String::new(),
            sub: String::new(),
            jti: Uuid::new_v4().to_string(),
            exp: Utc::now().timestamp() + 900,
            nbf: Utc::now().timestamp(),
            iat: Utc::now().timestamp(),
            scp: String::new(),
        }
    }
}

impl From<lerpz_model::User> for Claims {
    fn from(user: lerpz_model::User) -> Self {
        Claims {
            sub: user.id.to_string(),
            ..Default::default()
        }
    }
}
