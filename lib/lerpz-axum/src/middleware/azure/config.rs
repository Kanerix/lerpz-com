use axum::http::HeaderMap;
use regex::Regex;
use reqwest::StatusCode;
use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use jsonwebtoken::{DecodingKey, jwk::JwkSet};

use crate::error::HandlerError;

/// Azure configuration.
#[derive(Clone)]
pub struct AzureConfig {
    pub tenant_id: Cow<'static, str>,
    pub client_id: Cow<'static, str>,
    http_client: reqwest::Client,
    jwks_cache: Arc<RwLock<Option<JwksCache>>>,
}

/// A cache for JWKs (JSON Web Keys).
///
/// This cache is used to store the JWKs fetched from the Azure keys discovery
/// endpoint.
#[derive(Clone)]
struct JwksCache {
    keys: HashMap<String, DecodingKey>,
    expires_at: Instant,
}

impl AzureConfig {
    /// Create a new [`AzureConfig`].
    pub fn new(
        tenant_id: impl Into<Cow<'static, str>>,
        client_id: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            tenant_id: tenant_id.into(),
            client_id: client_id.into(),
            http_client: reqwest::Client::new(),
            jwks_cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Get the URL for the JWKs (JSON Web Keys) endpoint.
    pub fn get_jwks_url(&self) -> String {
        format!(
            "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
            &self.tenant_id
        )
    }

    /// Get the URL for the issuer endpoint.
    pub fn get_issuer_url(&self) -> String {
        format!("https://login.microsoftonline.com/{}/v2.0", &self.tenant_id)
    }

    /// Get a JWK (JSON Web Key) by its key ID.
    pub async fn get_jwk(&self, kid: String) -> Result<Option<DecodingKey>, HandlerError> {
        let cache = self.jwks_cache.read().await;
        if let Some(cached) = cache.as_ref() {
            if cached.expires_at > Instant::now() {
                self.fetch_jwks().await?;
            }

            if let Some(key) = cached.keys.get(&kid) {
                return Ok(Some(key.clone()));
            } else {
                return Ok(None);
            }
        } else {
            Ok(None)
        }
    }

    /// Fetch the JWKs (JSON Web Keys) from the Azure endpoint.
    ///
    /// This will read the cache-control header to determine how long the
    /// fetched keys are valid for. If this header is invalid it will defualt to
    /// 24 hours (86400 sec).
    async fn fetch_jwks(&self) -> Result<(), HandlerError> {
        let response = self
            .http_client
            .get(&self.get_jwks_url())
            .timeout(Duration::from_secs(10))
            .send()
            .await?;

        let expires_at = expires_at(response.headers()).unwrap_or(86400);

        let jwk_set: JwkSet = if response.status().is_success() {
            response.json().await?
        } else {
            return Err(HandlerError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWK not found",
                "Unable to fetch JWK set from Microsoft",
            ));
        };

        let mut map = HashMap::new();
        for key in jwk_set.keys {
            let decoding_key = DecodingKey::from_jwk(&key)?;
            let key_id = match key.common.key_id {
                Some(kid) => kid,
                None => continue,
            };
            map.insert(key_id, decoding_key);
        }

        let cached = JwksCache {
            keys: map,
            expires_at: Instant::now() + Duration::from_secs(expires_at),
        };

        let mut cache = self.jwks_cache.write().await;
        *cache = Some(cached);

        Ok(())
    }
}

static EXPIRES_AT_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|,\s*)max-age=(\d+)").unwrap());

fn expires_at(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("cache-control")
        .and_then(|v| v.to_str().ok())
        .and_then(|header_value| {
            EXPIRES_AT_REGEX
                .captures(header_value)
                .and_then(|caps| caps.get(1))
                .and_then(|m| m.as_str().parse().ok())
        })
}
