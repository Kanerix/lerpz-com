//! OpenAI client integration for `rig-core`.
//!
//! This module provides utilities for building an [`openai::Client`] that
//! routes every request through the Portkey AI gateway at `api_base`.
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
//! use lerpz_agent::client::build_client;
//! use secrecy::SecretString;
//! use std::sync::Arc;
//!
//! let client = build_client(
//!     &Arc::from("https://api.portkey.ai/v1"),
//!     &SecretString::new("pk-…".to_owned()),
//! )?;
//!
//! let agent = client.agent("gpt-4o").build();
//! ```

use std::sync::Arc;

use http::HeaderMap;
use rig::providers::openai;
use secrecy::{ExposeSecret, SecretString};

pub fn build_client(
    api_base: &Arc<str>,
    api_key: &SecretString,
    #[cfg(feature = "portkey")] provider: &Arc<str>,
) -> crate::Result<openai::Client> {
    let mut headers = HeaderMap::new();

    #[cfg(feature = "portkey")]
    {
        headers.insert(
            "x-portkey-api-key",
            api_key.expose_secret().parse().map_err(|e| {
                crate::Error::ClientBuild(format!("invalid x-portkey-api-key header value: {e}"))
            })?,
        );

        headers.insert(
            "x-portkey-provider",
            provider.parse().map_err(|e| {
                crate::Error::ClientBuild(format!("invalid x-portkey-provider header value: {e}"))
            })?,
        );
    }

    let client = openai::Client::builder()
        .api_key(api_key.expose_secret())
        .base_url(api_base.as_ref())
        .http_headers(headers)
        .build()
        .map_err(|e| {
            crate::Error::ClientBuild(format!("failed to build Portkey/rig client: {e}"))
        })?;

    Ok(client)
}
