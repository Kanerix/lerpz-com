//! A middleware that enables azure auth.
//!
//! ### Example
//!
//! ```rust
//! use axum::extract::FromRef;
//! use lerpz_axum::{
//!     error::{HandlerResult, HandlerError},
//!     middleware::azure::{AzureAccessToken, AzureConfig}
//! };
//!
//! #[derive(Clone)]
//! pub struct AppState {
//!     pub azure_config: AzureConfig,
//! }
//!
//! impl FromRef<AppState> for AzureConfig {
//!     fn from_ref(state: &AppState) -> Self {
//!         state.azure_config.clone()
//!     }
//! }
//!
//! async fn example_handler(
//!   token: AzureAccessToken,
//! ) -> HandlerResult<String> {
//!     if !token.has_scope("example") {
//!         return Err(HandlerError::unauthorized());
//!     }
//!
//!     Ok("You have the required scope!".to_string())
//! }
//! ```
//!
//! ### Note:
//!
//! This does not support multi-tenant applications (yet).

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use jsonwebtoken::{decode, decode_header};
use serde::{Deserialize, Deserializer};

use crate::error::HandlerError;

pub use config::*;
pub use validation::*;

mod config;
mod error;
mod validation;

/// A token representing a user in the Azure Entra system.
///
/// This is implemented using the [Micrsoft
/// Documentation](https://learn.microsoft.com/en-us/entra/identity-platform/access-tokens)
///
/// This can be extracted in any handler by adding it as a parameter.
///
/// ### Example
///
/// ```rust
/// use lerpz_axum::{
///     error::{HandlerResult, HandlerError},
///     middleware::azure::{AzureAccessToken, AzureConfig}
/// };
///
/// async fn example_handler(
///   token: AzureAccessToken,
/// ) -> HandlerResult<String> {
///     if !token.has_scope("example") {
///         return Err(HandlerError::unauthorized());
///     }
///
///     Ok("You have the required scope!".to_string())
/// }
/// ```
///
/// ### Note:
///
/// This does not support multi-tenant applications (yet).
#[derive(Debug, Deserialize)]
pub struct AzureAccessToken {
    /// Version of the Microsoft JWT scheme.
    ///
    /// The versions and respective JSON scheme can be found in
    /// [Microsoft Documentation](https://learn.microsoft.com/en-us/entra/identity-platform/security-tokens).
    ///
    /// ### Note:
    ///
    /// Only "v2.0" is supported.
    pub ver: String,

    /// Tenant ID of the token.
    ///
    /// This will be the tenant ID of the identity.
    pub tid: String,

    /// Issuer of the token.
    pub iss: String,
    /// Audience for which the token is intended.
    pub aud: String,
    /// Expiration time of the token (as a Unix timestamp).
    pub exp: usize,
    /// "not before" time of the token (as a Unix timestamp).
    pub nbf: usize,
    /// Issued at time of the token (as a Unix timestamp).
    pub iat: usize,
    /// Subject of the JWT (most often the identities object ID).
    pub sub: String,

    /// Scopes assigned to the token.
    #[serde(default, deserialize_with = "deserialize_space_separated_scopes")]
    pub scp: Vec<String>,
    /// Roles assigned to the token.
    #[serde(default)]
    pub roles: Vec<String>,

    /// Application ID.
    ///
    /// This is only present for v1.0 tokens. This has been replaced by the
    /// [`azp'] claim in the v2.0 scheme.
    pub appid: Option<String>,
    /// Application ID.
    ///
    /// This is only present for v2.0 tokens. This has replaced the [`appid']
    /// claim in the old v1.0 scheme.
    pub azp: Option<String>,

    /// Group object IDs (if groups claim is configured).
    #[serde(default)]
    pub groups: Vec<String>,

    // Other optional claims
    pub email: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub in_corp: Option<bool>,
    pub ipaddr: Option<String>,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub nonce: Option<String>,
    pub oid: Option<String>,
    pub preferred_username: Option<String>,
    pub pwd_exp: Option<i64>,
    pub pwd_url: Option<String>,
    pub upn: Option<String>,
}

/// Deserialize space-separated scopes into a Vec<String>.
///
/// If the field is missing or null, returns an empty Vec.
fn deserialize_space_separated_scopes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    Ok(opt
        .as_deref()
        .map(|s| {
            s.split_whitespace()
                .filter(|scope| !scope.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default())
}

impl AzureAccessToken {
    /// Check if the token has scope.
    pub fn has_scope(&self, scope: impl AsRef<str>) -> bool {
        let scope = scope.as_ref();
        self.scp.iter().any(|s| s == scope)
    }

    /// Check if the token has any of scopes.
    pub fn has_any_scope<T: AsRef<str>>(&self, scopes: &[T]) -> bool {
        scopes.iter().any(|scope| self.has_scope(scope))
    }

    /// Check if the token has scope.
    ///
    /// This will return [`HandlerError::unauthorized()`] if scope is not found.
    pub fn require_scope(&self, scope: impl AsRef<str>) -> Result<(), HandlerError> {
        self.has_scope(scope)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of scopes.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all scopes are not found.
    pub fn require_any_scope<T: AsRef<str>>(&self, scopes: &[T]) -> Result<(), HandlerError> {
        self.has_any_scope(scopes)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has role.
    pub fn has_role(&self, role: impl AsRef<str>) -> bool {
        let role = role.as_ref();
        self.roles.iter().any(|r| r == role)
    }

    /// Check if the token has any of roles.
    pub fn has_any_role<T: AsRef<str>>(&self, roles: &[T]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }

    /// Check if the token has role.
    ///
    /// This will return [`HandlerError::unauthorized()`] if role is not found.
    pub fn require_role(&self, role: impl AsRef<str>) -> Result<(), HandlerError> {
        self.has_role(role)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of roles.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all roles are not found.
    pub fn require_any_role<T: AsRef<str>>(&self, roles: &[T]) -> Result<(), HandlerError> {
        self.has_any_role(roles)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }
}

impl<S> FromRequestParts<S> for AzureAccessToken
where
    AzureConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = HandlerError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(HandlerError::unauthorized)?;

        let header = match decode_header(token) {
            Ok(header) => header,
            Err(err) => {
                tracing::debug!("failed to decode JWT token header: {err}");
                return Err(HandlerError::unauthorized());
            }
        };
        let kid = match header.kid {
            Some(kid) => kid,
            None => {
                tracing::debug!("JWT token did not provide a 'kid' in header");
                return Err(HandlerError::unauthorized());
            }
        };

        let config = AzureConfig::from_ref(state);
        let decoding_key = match config.find_jwk(&kid).await {
            Ok(Some(key)) => key,
            Ok(None) => {
                tracing::warn!("unknown key ID: {}", kid);
                return Err(HandlerError::unauthorized());
            }
            Err(err) => {
                tracing::error!("failed to find JWK: {err}");
                return Err(HandlerError::from(err));
            }
        };

        let validation = get_token_validation(&config);
        let token_data = match decode::<AzureAccessToken>(token, &decoding_key, &validation) {
            Ok(token) => token,
            Err(err) => {
                tracing::trace!("validation of JWT claims failed: {err}");
                return Err(HandlerError::unauthorized());
            }
        };

        if !config.validate_azure_claims(&token_data.claims) {
            tracing::trace!("validation of azure claims failed");
            return Err(HandlerError::unauthorized());
        }

        Ok(token_data.claims)
    }
}
