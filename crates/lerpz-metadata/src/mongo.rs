use mongodb::bson::oid::ObjectId;

use crate::{MetadataClient, error::Result, models::Metadata};

pub struct MongoClient {
    client: mongodb::Client,
}

const ITEMS_PER_PAGE: u32 = 20;

impl MongoClient {
    pub async fn new(uri: &str) -> Result<Self> {
        let client = mongodb::Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }
}

impl MetadataClient for MongoClient {
    fn read(&self) -> Result<Metadata> {
        todo!()
    }

    fn write(&self, metadta: Metadata) -> Result<ObjectId> {
        todo!()
    }

    fn update(&self, id: ObjectId, updates: Metadata) -> Result<ObjectId> {
        todo!()
    }

    fn delete(&self, id: ObjectId) -> Result<Metadata> {
        todo!()
    }
}
