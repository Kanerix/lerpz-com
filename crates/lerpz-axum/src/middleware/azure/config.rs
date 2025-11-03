use axum::http::HeaderMap;
use regex::Regex;
use std::{
    borrow::Cow,
    sync::{Arc, LazyLock},
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

use jsonwebtoken::{
    DecodingKey,
    jwk::{Jwk, JwkSet},
};

use super::error::Result;

/// Azure configuration.
pub struct AzureConfig {
    pub tenant_id: Cow<'static, str>,
    pub client_id: Cow<'static, str>,
    pub issuer: String,
    jwks_url: String,
    jwks_cache: JwksCache,
    http_client: reqwest::Client,
}

/// A cache for JWKs (JSON Web Keys).
///
/// This cache is used to store the JWKs fetched from the Azure keys discovery
/// endpoint. This is wrapped in an [`std::sync::Arc`] so it can be safely used
/// in multithreaded environments.
#[derive(Clone)]
struct JwksCache {
    inner: Arc<RwLock<JwksCacheInner>>,
}

struct JwksCacheInner {
    jwks: JwkSet,
    expires_at: Instant,
}

impl JwksCache {
    fn new(jwks: JwkSet, cache_control: u64) -> JwksCache {
        let expires_at = Instant::now() + Duration::from_secs(cache_control);
        let cache = JwksCacheInner { jwks, expires_at };

        Self {
            inner: Arc::new(RwLock::new(cache)),
        }
    }

    /// Check if the cache needs to be refreshed.
    pub async fn is_valid(&self) -> bool {
        let cache = self.inner.read().await;
        cache.expires_at > Instant::now()
    }

    /// Refresh the JWK cache.
    pub async fn refresh(&self, http_client: &reqwest::Client, jwks_url: &str) -> Result<()> {
        let mut cache = self.inner.write().await;
        let (jwks, cache_control) = fetch_jwks(http_client, jwks_url).await?;
        let expires_at = Instant::now() + Duration::from_secs(cache_control);
        cache.jwks = jwks;
        cache.expires_at = expires_at;
        Ok(())
    }

    /// Get a JWK by its key ID.
    pub async fn find_jwk(
        &self,
        kid: String,
        http_client: &reqwest::Client,
        jwks_url: &str,
    ) -> Result<Option<Jwk>> {
        if !self.is_valid().await {
            self.refresh(http_client, jwks_url).await?;
        }
        let cache = self.inner.read().await;
        if let Some(key) = cache.jwks.find(&kid) {
            Ok(Some(key.clone()))
        } else {
            Ok(None)
        }
    }
}

impl AzureConfig {
    /// Create a new [`AzureConfig`].
    pub async fn new(
        tenant_id: impl Into<Cow<'static, str>>,
        client_id: impl Into<Cow<'static, str>>,
    ) -> Result<Self> {
        let tenant_id = tenant_id.into();
        let client_id = client_id.into();
        let issuer = format!("https://login.microsoftonline.com/{}/v2.0", &tenant_id);

        let http_client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let jwks_url = format!(
            "https://login.microsoftonline.com/{}/discovery/v2.0/keys",
            &tenant_id
        );

        let (jwks, cache_control) = fetch_jwks(&http_client, &jwks_url).await?;
        let jwks_cache = JwksCache::new(jwks, cache_control);

        Ok(Self {
            tenant_id,
            client_id,
            issuer,
            jwks_url,
            jwks_cache,
            http_client,
        })
    }

    /// Validate claims in an Azure access token.
    pub fn validate_token_claims(&self, claims: &super::AzureAccessToken) -> bool {
        if claims.ver.as_deref() != Some("2.0") {
            return false;
        }

        if let Some(tid) = &claims.tid {
            if tid.as_str() != self.tenant_id {
                return false;
            }
        } else {
            return false;
        }

        !claims.sub.is_empty()
    }

    pub async fn find_jwk(&self, kid: String) -> Result<Option<DecodingKey>> {
        let jwk = self
            .jwks_cache
            .find_jwk(kid, &self.http_client, &self.jwks_url)
            .await?;
        if let Some(k) = jwk {
            Ok(Some(DecodingKey::from_jwk(&k)?))
        } else {
            Ok(None)
        }
    }
}

/// Fetch the JWKs (JSON Web Keys) from the Azure endpoint.
///
/// This will read the Cache-Control header to determine how long the fetched
/// keys are valid for. If this header is invalid or missing it will defualt to
/// 24 hours (86400 sec).
pub async fn fetch_jwks(http_client: &reqwest::Client, jwks_url: &str) -> Result<(JwkSet, u64)> {
    let response = http_client.get(jwks_url).send().await?;
    let cache_control = get_cache_control(response.headers()).unwrap_or(86400);
    Ok((response.json().await?, cache_control))
}

static CACHE_CONTROL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|,\s*)max-age=(\d+)").unwrap());

/// Read the Cache-Control header and return its value.
///
/// This will return [`None`] when the header is not present or invalid.
fn get_cache_control(headers: &HeaderMap) -> Option<u64> {
    headers
        .get("cache-control")
        .and_then(|v| v.to_str().ok())
        .and_then(|header_value| {
            CACHE_CONTROL_REGEX
                .captures(header_value)
                .and_then(|caps| caps.get(1))
                .and_then(|m| m.as_str().parse().ok())
        })
}
