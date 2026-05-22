use async_openai::config::Config;
use axum::http::HeaderMap;
use secrecy::{ExposeSecret, SecretString};

#[derive(Clone, Debug)]
pub(crate) struct PortkeyConfig {
    pub api_base: String,
    pub api_key: SecretString,
    pub api_provider: String,
}

impl Config for PortkeyConfig {
    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Content-Type",
            "application/json"
                .parse()
                .expect("Invalid Content-Type header value for PortkeyConfig"),
        );
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
