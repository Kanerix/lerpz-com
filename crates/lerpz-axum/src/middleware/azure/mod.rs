//! A middleware that enables azure auth.
//!
//! ### Example
//!
//! ```rust
//! use axum::extract::FromRef;
//! use lerpz_axum::{
//!     middleware::azure::{AzureAccessToken, AzureConfig},
//!     problem::{HandlerResult, Problem},
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
//! async fn example_handler(token: AzureAccessToken) -> HandlerResult<String> {
//!     if !token.has_scope("example") {
//!         return Err(Problem::unauthorized());
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
    http::{HeaderMap, header::AUTHORIZATION, request::Parts},
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Deserializer};
use std::{ops::Deref, sync::Arc};

use crate::problem::Problem;

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
/// Claims are shared through an [`Arc`]. Cloning a token does not copy its claims.
/// Fields can be borrowed through [`Deref`]; clone individual fields when ownership
/// is needed.
///
/// ### Example
///
/// ```rust
/// use lerpz_axum::{
///     middleware::azure::AzureAccessToken,
///     problem::{HandlerResult, Problem},
/// };
///
/// async fn example_handler(token: AzureAccessToken) -> HandlerResult<String> {
///     if !token.has_scope("example") {
///         return Err(Problem::unauthorized());
///     }
///
///     Ok("You have the required scope!".to_string())
/// }
/// ```
///
/// ### Note:
///
/// This does not support multi-tenant applications (yet).
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "AzureAccessTokenClaims")]
pub struct AzureAccessToken(Arc<AzureAccessTokenClaims>);

impl From<AzureAccessTokenClaims> for AzureAccessToken {
    fn from(claims: AzureAccessTokenClaims) -> Self {
        Self(Arc::new(claims))
    }
}

impl Deref for AzureAccessToken {
    type Target = AzureAccessTokenClaims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Claims shared by an [`AzureAccessToken`].
#[derive(Debug, Deserialize)]
pub struct AzureAccessTokenClaims {
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
    /// This will return [`Problem::unauthorized()`] if scope is not found.
    pub fn require_scope(&self, scope: impl AsRef<str>) -> Result<(), Problem> {
        self.has_scope(scope)
            .then_some(())
            .ok_or(Problem::unauthorized())
    }

    /// Check if the token has any of scopes.
    ///
    /// This will return [`Problem::unauthorized()`] if all scopes are not found.
    pub fn require_any_scope<T: AsRef<str>>(&self, scopes: &[T]) -> Result<(), Problem> {
        self.has_any_scope(scopes)
            .then_some(())
            .ok_or(Problem::unauthorized())
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
    /// This will return [`Problem::unauthorized()`] if role is not found.
    pub fn require_role(&self, role: impl AsRef<str>) -> Result<(), Problem> {
        self.has_role(role)
            .then_some(())
            .ok_or(Problem::unauthorized())
    }

    /// Check if the token has any of roles.
    ///
    /// This will return [`Problem::unauthorized()`] if all roles are not found.
    pub fn require_any_role<T: AsRef<str>>(&self, roles: &[T]) -> Result<(), Problem> {
        self.has_any_role(roles)
            .then_some(())
            .ok_or(Problem::unauthorized())
    }
}

/// A validated Entra access token in its original encoded form, without `Bearer `.
///
/// Extract with `RawAzureToken(token): RawAzureToken`. The [`Arc`]-shared
/// [`SecretString`] redacts debug output and zeroises its memory when the last
/// owner is dropped. Scope, role and ownership checks remain the caller's
/// responsibility.
///
/// Can be used alongside [`AzureAccessToken`] in either order. Both extractors
/// require one Authorization header and reuse validation within the request.
pub struct RawAzureToken(pub Arc<SecretString>);

impl<S> FromRequestParts<S> for RawAzureToken
where
    AzureConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Problem;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = AzureConfig::from_ref(state);
        Ok(Self(extract_token(parts, &config).await?.raw))
    }
}

impl<S> FromRequestParts<S> for AzureAccessToken
where
    AzureConfig: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Problem;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = AzureConfig::from_ref(state);
        Ok(extract_token(parts, &config).await?.claims)
    }
}

#[derive(Clone)]
struct ValidatedAzureToken {
    raw: Arc<SecretString>,
    claims: AzureAccessToken,
}

async fn extract_token(
    parts: &mut Parts,
    config: &AzureConfig,
) -> Result<ValidatedAzureToken, Problem> {
    let token = bearer_token(&mut parts.headers)?;
    if let Some(cached) = parts.extensions.get::<ValidatedAzureToken>()
        && cached.raw.expose_secret() == token
        && cached.claims.aud == config.aud()
        && cached.claims.iss == config.iss()
        && config.validate_azure_claims(&cached.claims)
    {
        return Ok(cached.clone());
    }

    let claims = validate_azure_token(token, config).await?;
    let validated = ValidatedAzureToken {
        raw: Arc::new(SecretString::from(token)),
        claims,
    };
    parts.extensions.insert(validated.clone());
    Ok(validated)
}

fn bearer_token(headers: &mut HeaderMap) -> Result<&str, Problem> {
    if headers.get_all(AUTHORIZATION).iter().count() != 1 {
        return Err(Problem::unauthorized());
    }
    let authorization = headers
        .get_mut(AUTHORIZATION)
        .ok_or_else(Problem::unauthorized)?;
    authorization.set_sensitive(true);
    authorization
        .to_str()
        .ok()
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or_else(Problem::unauthorized)
}

#[cfg(test)]
mod tests {
    use axum::http::{HeaderValue, Request};
    use jsonwebtoken::jwk::JwkSet;
    use serde_json::{Value, json};
    use std::borrow::Cow;

    use super::{config::JwksCache, *};

    fn azure_config() -> AzureConfig {
        AzureConfig {
            inner: Arc::new(AzureConfigInner {
                tenant_id: Cow::Borrowed("tenant"),
                client_id: Cow::Borrowed("client"),
                issuer: Cow::Borrowed("https://login.microsoftonline.com/tenant/v2.0"),
                jwks_url: String::new(),
                jwks_cache: JwksCache::new(JwkSet { keys: Vec::new() }, 3600),
                http_client: reqwest::Client::new(),
            }),
        }
    }

    fn claims_json() -> Value {
        json!({
            "ver": "2.0",
            "tid": "tenant",
            "iss": "https://login.microsoftonline.com/tenant/v2.0",
            "aud": "client",
            "exp": 4_000_000_000_u64,
            "nbf": 0,
            "iat": 0,
            "sub": "subject",
            "scp": "read write",
            "roles": ["reader"],
            "groups": ["group"]
        })
    }

    fn cached_request(claims: Value) -> (Parts, ValidatedAzureToken) {
        let cached = ValidatedAzureToken {
            raw: Arc::new(SecretString::from("cached-token")),
            claims: serde_json::from_value(claims).expect("valid test claims"),
        };
        let (mut parts, ()) = Request::builder()
            .header(AUTHORIZATION, "Bearer cached-token")
            .body(())
            .expect("valid test request")
            .into_parts();
        parts.extensions.insert(cached.clone());
        (parts, cached)
    }

    #[test]
    fn token_clone_shares_claims() {
        let token: AzureAccessToken =
            serde_json::from_value(claims_json()).expect("valid test claims");
        let cloned = token.clone();

        assert!(Arc::ptr_eq(&token.0, &cloned.0));
        assert_eq!(cloned.sub, "subject");
        assert!(cloned.has_scope("read"));
        assert!(cloned.has_scope("write"));
        assert!(cloned.has_role("reader"));
        assert_eq!(cloned.groups, ["group"]);
    }

    #[test]
    fn missing_and_null_scopes_remain_empty() {
        for scopes in [None, Some(Value::Null)] {
            let mut claims = claims_json();
            claims.as_object_mut().expect("claims object").remove("scp");
            if let Some(scopes) = scopes {
                claims["scp"] = scopes;
            }
            let token: AzureAccessToken =
                serde_json::from_value(claims).expect("valid test claims");
            assert!(token.scp.is_empty());
        }
    }

    #[tokio::test]
    async fn extractors_share_cached_data_in_either_order() {
        let config = azure_config();
        for raw_first in [false, true] {
            let (mut parts, cached) = cached_request(claims_json());
            if raw_first {
                let raw = RawAzureToken::from_request_parts(&mut parts, &config)
                    .await
                    .expect("cached raw token");
                assert!(Arc::ptr_eq(&raw.0, &cached.raw));
            }

            let claims = AzureAccessToken::from_request_parts(&mut parts, &config)
                .await
                .expect("cached claims");
            let raw = RawAzureToken::from_request_parts(&mut parts, &config)
                .await
                .expect("cached raw token");
            let repeated = AzureAccessToken::from_request_parts(&mut parts, &config)
                .await
                .expect("repeated cached claims");

            assert!(Arc::ptr_eq(&claims.0, &cached.claims.0));
            assert!(Arc::ptr_eq(&repeated.0, &claims.0));
            assert!(Arc::ptr_eq(&raw.0, &cached.raw));
            assert_eq!(raw.0.expose_secret(), "cached-token");
            assert!(parts.headers[AUTHORIZATION].is_sensitive());
        }
    }

    #[tokio::test]
    async fn cached_token_requires_the_same_single_header() {
        let config = azure_config();
        for headers in [
            vec![],
            vec!["Bearer other-token"],
            vec!["Bearer cached-token", "Bearer cached-token"],
        ] {
            let (mut parts, _) = cached_request(claims_json());
            parts.headers.remove(AUTHORIZATION);
            for header in headers {
                parts
                    .headers
                    .append(AUTHORIZATION, HeaderValue::from_static(header));
            }

            assert!(
                AzureAccessToken::from_request_parts(&mut parts, &config)
                    .await
                    .is_err()
            );
            assert!(
                RawAzureToken::from_request_parts(&mut parts, &config)
                    .await
                    .is_err()
            );
        }
    }

    #[tokio::test]
    async fn cached_token_rechecks_claims_and_config() {
        let config = azure_config();
        for (field, value) in [
            ("aud", "other-client"),
            ("iss", "other-issuer"),
            ("tid", "other-tenant"),
            ("ver", "1.0"),
            ("sub", ""),
        ] {
            let mut claims = claims_json();
            claims[field] = Value::from(value);
            let (mut parts, _) = cached_request(claims);

            assert!(
                AzureAccessToken::from_request_parts(&mut parts, &config)
                    .await
                    .is_err()
            );
            assert!(
                RawAzureToken::from_request_parts(&mut parts, &config)
                    .await
                    .is_err()
            );
        }
    }
}
