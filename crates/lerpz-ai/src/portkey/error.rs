/// A type alias for handling Portkey client construction errors.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when wiring up a client for the Portkey gateway.
///
/// The underlying causes come from feature-gated dependencies, so they are
/// kept as their rendered message rather than as a [`source`](std::error::Error::source).
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// A header the gateway requires could not be turned into a header value.
    #[error("invalid {name} header value: {reason}")]
    InvalidHeader {
        /// The name of the offending header.
        name: &'static str,
        /// Why the value was rejected.
        reason: String,
    },

    /// The gateway client could not be constructed.
    #[error("failed to build gateway client: {0}")]
    ClientBuild(String),
}
