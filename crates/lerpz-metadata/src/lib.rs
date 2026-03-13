//! Working with content metadata in a mongo database.

pub mod error;
pub mod models;
pub mod mongo;

pub use error::Result;
pub use models::Metadata;
pub use mongo::MongoClient;

use mongodb::bson::oid::ObjectId;

trait MetadataClient {
    fn read(&self) -> Result<Metadata>;
    fn write(&self, metadta: Metadata) -> Result<ObjectId>;
    fn update(&self, id: ObjectId, updates: Metadata) -> Result<ObjectId>;
    fn delete(&self, id: ObjectId) -> Result<Metadata>;
}
