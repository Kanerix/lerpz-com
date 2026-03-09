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
    fn query(&self) -> Result<Metadata> {
        todo!()
    }
    fn query_page(&self, _page: u32) -> Result<Vec<Metadata>> {
        todo!()
    }
}
