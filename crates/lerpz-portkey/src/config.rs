use async_openai::config::Config;
use http::HeaderMap;
use secrecy::{ExposeSecret, SecretString};

/// An [`async-openai`](async_openai) [`Config`] that routes every request
/// through the [Portkey](https://portkey.ai) AI gateway.
///
/// Portkey exposes an OpenAI-compatible REST API but expects a couple of extra
/// headers (the workspace API key and the upstream provider to route to), which
/// this config injects on every request.
#[derive(Clone, Debug)]
pub struct PortkeyConfig {
    pub api_base: String,
    pub api_key: SecretString,
    pub api_provider: String,
}

impl Config for PortkeyConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        // NOTE: don't set `Content-Type` here. These headers are applied to
        // every request, but `async-openai`/`reqwest` set the correct
        // `Content-Type` per request (`application/json` for JSON endpoints,
        // `multipart/form-data` for form uploads such as `POST /videos`).
        // Forcing `application/json` here appends a second, conflicting
        // `Content-Type` to multipart requests, so Portkey can't parse the body
        // (it never sees the `model` field) and rejects the call with
        // "Invalid provider passed".
        headers.insert(
            "x-portkey-api-key",
            self.api_key
                .expose_secret()
                .parse()
                .expect("Invalid x-portkey-api-key header value for PortkeyConfig"),
        );
        headers.insert(
            "x-portkey-provider",
            self.api_provider
                .parse()
                .expect("Invalid x-portkey-config header value for PortkeyConfig"),
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
