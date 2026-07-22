use async_openai::config::Config;
use http::HeaderMap;
use secrecy::{ExposeSecret, SecretString};

/// An [`async-openai`](async_openai) [`Config`] that routes every request
/// through the [Portkey](https://portkey.ai) AI gateway.
///
/// Portkey exposes an OpenAI-compatible REST API but expects the workspace API
/// key as an extra header, which this config injects on every request. The
/// upstream provider is selected per request via the `@<provider-slug>/<model>`
/// model-catalog syntax rather than a header.
#[derive(Clone, Debug)]
pub struct PortkeyConfig {
    pub api_base: String,
    pub api_key: SecretString,
}

impl PortkeyConfig {
    /// Creates a new [`PortkeyConfig`] with the given API base URL and API key.
    #[inline]
    pub fn new(api_base: String, api_key: SecretString) -> Self {
        Self { api_base, api_key }
    }
}

impl Config for PortkeyConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "x-portkey-api-key",
            self.api_key
                .expose_secret()
                .parse()
                .expect("Invalid x-portkey-api-key header value for PortkeyConfig"),
        );
        headers.insert(
            "x-portkey-strict-open-ai-compliance",
            "false".parse().expect(
                "Invalid x-portkey-strict-open-ai-compliance header value for PortkeyConfig",
            ),
        );

        headers
    }

    fn api_key(&self) -> &SecretString {
        &self.api_key
    }

    fn api_base(&self) -> &str {
        &self.api_base
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.api_base, path)
    }

    fn query(&self) -> Vec<(&str, &str)> {
        Vec::new()
    }
}
