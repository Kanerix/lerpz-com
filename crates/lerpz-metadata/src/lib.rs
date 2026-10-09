//! Working with content metadata.
//!
//! The models and the [`MetadataClient`] trait are always available. Enable the
//! `postgres` feature for the [`postgres`] module, which implements the trait
//! against a database.

pub mod error;
pub mod models;
pub mod utils;

#[cfg(feature = "postgres")]
pub mod postgres;

pub use error::Result;
pub use models::{Metadata, MetadataUpdates};
pub use utils::{public_url, save_to_s3};

#[cfg(feature = "postgres")]
pub use postgres::Client;

use uuid::Uuid;

use models::StorageMetadata;

/// Discriminant used to identify which metadata table to operate on for
/// queries that cannot infer the type from a [`Metadata`] value alone (i.e.
/// `get`, `update` and `delete`).
pub enum MetadataKind {
    Image,
    Video,
    Audio,
}

pub trait MetadataClient {
    /// Fetch a single metadata record by its id.
    fn get(&self, id: Uuid, kind: MetadataKind) -> impl Future<Output = Result<Metadata>>;
    /// Insert a new metadata record and return its generated id.
    fn insert(&self, metadata: Metadata) -> impl Future<Output = Result<Uuid>>;
    /// Apply a partial update to an existing record.
    ///
    /// Groups left as `None` in `updates` keep the values they already have.
    fn update(
        &self,
        id: Uuid,
        kind: MetadataKind,
        updates: MetadataUpdates,
    ) -> impl Future<Output = Result<Uuid>>;
    /// Delete a record and return the deleted value.
    fn delete(&self, id: Uuid, kind: MetadataKind) -> impl Future<Output = Result<Metadata>>;
}

/// The provider and path of a stored object.
#[derive(Debug, PartialEq, Eq)]
pub enum StorageLocation<'a> {
    S3 { bucket: &'a str, key: &'a str },
    AzureBlob { container: &'a str, blob: &'a str },
}

/// Provides a stored object's location without assuming its storage provider.
pub trait StorageLocationSource {
    fn storage_location(&self) -> StorageLocation<'_>;
}

impl StorageLocationSource for StorageMetadata {
    fn storage_location(&self) -> StorageLocation<'_> {
        match self {
            Self::S3 { bucket, key } => StorageLocation::S3 { bucket, key },
            Self::AzureBlob { container, blob } => StorageLocation::AzureBlob { container, blob },
        }
    }
}

impl StorageLocationSource for Metadata {
    fn storage_location(&self) -> StorageLocation<'_> {
        let storage = match self {
            Self::Image { storage, .. }
            | Self::Video { storage, .. }
            | Self::Audio { storage, .. } => storage,
        };
        storage.storage_location()
    }
}

impl StorageLocationSource for (&str, &str) {
    fn storage_location(&self) -> StorageLocation<'_> {
        StorageLocation::S3 {
            bucket: self.0,
            key: self.1,
        }
    }
}
