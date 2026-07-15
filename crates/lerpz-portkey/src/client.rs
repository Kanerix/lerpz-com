//! [`rig-core`](rig_core) client integration for the Portkey AI gateway.
//!
//! This module provides utilities for building an [`openai::Client`] that
//! routes every request through the Portkey AI gateway at `base_url`.
//!
//! # Portkey
//!
//! Portkey acts as an AI gateway that proxies requests to upstream LLM
//! providers (OpenAI, Anthropic, etc.) while adding observability, caching,
//! and routing features. It uses a standard OpenAI-compatible REST API but
//! requires two extra headers:
//!
//! - `x-portkey-api-key` — your Portkey workspace API key
//! - `x-portkey-provider` — the upstream provider to route to
//!
//! # Example
//!
//! ```ignore
//! use lerpz_portkey::build_client;
//! use secrecy::SecretString;
//!
//! let client = build_client(
//!     "https://api.portkey.ai/v1",
//!     &SecretString::from("pk-…".to_owned()),
//!     "openai",
//! )?;
//!
//! let agent = client.agent("gpt-4o").build();
//! ```

use http::HeaderMap;
use rig_core::providers::openai;
use secrecy::{ExposeSecret, SecretString};

/// Builds a [`rig-core`](rig_core) [`openai::Client`] pointed at the Portkey AI
/// gateway.
///
/// The returned client sends the `x-portkey-api-key` and `x-portkey-provider`
/// headers on every request so Portkey can authenticate and route to the
/// chosen upstream provider.
pub fn build_client(
    base_url: &str,
    api_key: &SecretString,
    provider: &str,
) -> anyhow::Result<openai::Client> {
    let mut headers = HeaderMap::new();

    headers.insert(
        "x-portkey-api-key",
        api_key
            .expose_secret()
            .parse()
            .map_err(|e| anyhow::anyhow!(format!("invalid x-portkey-api-key header value: {e}")))?,
    );
    headers.insert(
        "x-portkey-provider",
        provider.parse().map_err(|e| {
            anyhow::anyhow!(format!("invalid x-portkey-provider header value: {e}"))
        })?,
    );

    openai::Client::builder()
        .api_key(api_key.expose_secret())
        .base_url(base_url)
        .http_headers(headers)
        .build()
        .map_err(|e| anyhow::anyhow!(format!("failed to build Agent client: {e}")))
}
