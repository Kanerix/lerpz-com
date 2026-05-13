//! Working with content metadata in a postgres database.

pub mod error;
pub mod models;
pub mod postgres;
pub mod utils;

pub use error::Result;
pub use models::Metadata;
pub use postgres::Client;
pub use utils::save_to_s3;
pub use self::MetadataClient as MetadataClientExt;

use uuid::Uuid;

/// Discriminant used to identify which metadata table to operate on for
/// queries that cannot infer the type from a [`Metadata`] value alone (i.e.
/// `get` and `delete`).
pub enum MetadataKind {
    Chat,
    Image,
    Video,
    Audio,
}

pub trait MetadataClient {
    /// Fetch a single metadata record by its id.
    async fn get(&self, id: Uuid, kind: MetadataKind) -> Result<Metadata>;
    /// Insert a new metadata record and return its generated id.
    async fn insert(&self, metadata: Metadata) -> Result<Uuid>;
    /// Overwrite the mutable fields of an existing record.
    async fn update(&self, id: Uuid, updates: Metadata) -> Result<Uuid>;
    /// Delete a record and return the deleted value.
    async fn delete(&self, id: Uuid, kind: MetadataKind) -> Result<Metadata>;
}
