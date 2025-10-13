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
mod validation;

/// A token representing a user in the Azure Entra system.
/// 
/// This can be extracted in any handler by adding it as a parameter.
/// 
/// ### Example
///
/// ```rust
/// async fn example_handler(
///   token: AzureAccessToken,
/// ) -> HandlerResult<String> {
///     if !token.has_scope("example/scope") {
///         Err(HandlerError::unauthorized())
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
    /// Issuer of the token.
    pub iss: String,
    /// Audience for which the token is intended.
    pub aud: String,
    /// Expiration time of the token (as a Unix timestamp).
    pub exp: usize,
    /// "not before" time of the token (as a Unix timestamp).
    pub nbf: Option<usize>,
    /// Issued at time of the token (as a Unix timestamp).
    pub iat: Option<usize>,
    /// Subject of the JWT (Often the user).
    pub sub: Option<String>,

    /// Version of the Microsoft JWT scheme.
    /// 
    /// The versions and their JSON scheme can be found in [Microsoft
    /// Documentation](https://learn.microsoft.com/en-us/entra/identity-platform/security-tokens).
    /// 
    /// ### Note:
    /// 
    /// Only "v2.0" is supported.
    pub ver: Option<String>,
    /// Scopes assigned to the token.
    #[serde(default, deserialize_with = "deserialize_space_separated_scopes")]
    pub scp: Vec<String>,
    /// Roles assigned to the token.
    pub roles: Option<Vec<String>>,
    /// Tenant ID of the user.
    pub tid: Option<String>,
    /// App ID of the application.
    pub appid: Option<String>,

    /// Unique identifier for the user.
    ///
    /// This is a optional claim that can be enabled in the app registration.
    pub nonce: Option<String>,
    /// Preferred username of the user.
    ///
    /// This is a optional claim that can be enabled in the app registration.
    pub name: Option<String>,
    /// Email of the user.
    ///
    /// This is a optional claim that can be enabled in the app registration.
    pub email: Option<String>,

    // Optional claims
    pub upn: Option<String>,
    pub ipaddr: Option<String>,
    pub pwd_exp: Option<i64>,
    pub pwd_url: Option<String>,
    pub in_corp: Option<bool>,
    pub nickname: Option<String>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
}

/// Used to deserialize the [`AzureToken::scp`].
fn deserialize_space_separated_scopes<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::deserialize(deserializer)?
        .map(|s: String| {
            s.split_whitespace()
                .map(|scope| scope.to_string())
                .collect()
        })
        .unwrap_or_default())
}

impl AzureAccessToken {
    /// Check if the token has scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scp.iter().any(|s| s == scope)
    }

    /// Check if the token has any of scopes.
    pub fn has_any_scope(&self, scopes: &[&str]) -> bool {
        scopes.iter().any(|scope| self.has_scope(scope))
    }

    /// Check if the token has scope.
    ///
    /// This will return [`HandlerError::unauthorized()`] if scope is not found.
    pub fn has_scope_or_unauthorized(&self, scope: &str) -> Result<(), HandlerError> {
        self.has_scope(scope)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of scopes.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all scopes are not found.
    pub fn has_any_scope_or_unauthorized(&self, scopes: &[&str]) -> Result<(), HandlerError> {
        self.has_any_scope(scopes)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has role.
    pub fn has_role(&self, role: &str) -> bool {
        self.roles
            .as_ref()
            .map(|roles| roles.contains(&role.to_string()))
            .unwrap_or(false)
    }

    /// Check if the token has any of roles.
    pub fn has_any_role(&self, roles: &[&str]) -> bool {
        roles.iter().any(|role| self.has_role(role))
    }

    /// Check if the token has role.
    ///
    /// This will return [`HandlerError::unauthorized()`] if role is not found.
    pub fn has_role_or_unauthorized(&self, role: &str) -> Result<(), HandlerError> {
        self.has_role(role)
            .then_some(())
            .ok_or(HandlerError::unauthorized())
    }

    /// Check if the token has any of roles.
    ///
    /// This will return [`HandlerError::unauthorized()`] if all roles are not found.
    pub fn has_any_role_or_unauthorized(&self, roles: &[&str]) -> Result<(), HandlerError> {
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
            .ok_or_else(|| HandlerError::unauthorized())?;

        let header = decode_header(token).map_err(HandlerError::unauthorized_with_error)?;
        let kid = header.kid.ok_or(HandlerError::unauthorized())?;

        let config = AzureConfig::from_ref(state);
        let decoding_key = config
            .get_jwk(kid)
            .await?
            .ok_or(HandlerError::unauthorized())?;

        let validation = get_token_validation(&config);
        let token_data = decode::<AzureAccessToken>(token, &decoding_key, &validation)
            .map_err(HandlerError::unauthorized_with_error)?;

        if !azure_claims_validation(&config, &token_data.claims) {
            return Err(HandlerError::unauthorized());
        }

        Ok(token_data.claims)
    }
}
