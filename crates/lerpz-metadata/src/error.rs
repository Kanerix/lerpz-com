use uuid::Uuid;

/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the metadata module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// A database error from sqlx.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// A generic S3 error.
    #[error(transparent)]
    S3(#[from] aws_sdk_s3::Error),

    /// The metadata is invalid for current operation.
    #[error("invalid metadata: {0}")]
    InvalidMetadata(String),

    /// No record with the given id was found.
    #[error("metadata not found: {0}")]
    NotFound(Uuid),
}
