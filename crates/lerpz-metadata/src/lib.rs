//! Working with content metadata in a mongo database.

pub mod error;
pub mod models;
pub mod mongo;

pub use error::Result;
pub use models::Metadata;
pub use mongo::MongoClient;

trait MetadataClient {
    fn query(&self) -> Result<Metadata>;
    fn query_page(&self, page: u32) -> Result<Vec<Metadata>>;
}
