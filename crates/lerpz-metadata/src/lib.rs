//! Working with content metadata in a postgres database.

pub mod error;
pub mod models;
pub mod postgres;
pub mod utils;

pub use self::MetadataClient as MetadataClientExt;
pub use error::Result;
pub use models::Metadata;
pub use postgres::Client;
pub use utils::save_to_s3;

use uuid::Uuid;

/// Discriminant used to identify which metadata table to operate on for
/// queries that cannot infer the type from a [`Metadata`] value alone (i.e.
/// `get` and `delete`).
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
    /// Overwrite the mutable fields of an existing record.
    fn update(&self, id: Uuid, updates: Metadata) -> impl Future<Output = Result<Uuid>>;
    /// Delete a record and return the deleted value.
    fn delete(&self, id: Uuid, kind: MetadataKind) -> impl Future<Output = Result<Metadata>>;
}
