use axum::http::StatusCode;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("invalid FORGE_URL")]
    InvalidOrigin,
    #[error("Forge client error")]
    Client(#[from] reqwest::Error),
    #[error("unexpected status {actual}, expected {expected}")]
    UnexpectedStatus {
        actual: StatusCode,
        expected: StatusCode,
    },
    #[error("unexpected response body")]
    UnexpectedBody,
    #[error("invalid agent response")]
    InvalidResponse,
}
