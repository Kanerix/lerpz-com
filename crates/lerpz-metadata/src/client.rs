use mongodb::bson::doc;

use crate::error::Error;

pub struct MetadataClient {
    client: mongodb::Client,
}

impl MetadataClient {
    pub async fn new(uri: &str) -> Result<Self, Error> {
        let client = mongodb::Client::with_uri_str(uri).await?;
        Ok(Self { client })
    }

    pub async fn get_metadata<T>(&self, id: &str) -> Result<Option<T>, Error>
    where
        T: serde::de::DeserializeOwned + Unpin + Send + Sync,
    {
        let collection = self.client.database("lerpz").collection("metadata");
        let filter = doc! { "_id": id };
        let result = collection.find_one(filter, None).await?;
        Ok(result)
    }
}
