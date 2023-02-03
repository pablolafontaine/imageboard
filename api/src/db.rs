use bson::from_document;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions,
    Client, Collection,
};
use std::{error::Error, str::FromStr};
use types::{GenerationError, PostResponse};

#[derive(Clone)]
pub struct Db(Collection<Document>);

impl Db {
    pub async fn new(uri: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Db(client.database("ImageBoard").collection("Images")))
    }
    pub async fn add_image(&self, doc: &Document) -> Result<String, Box<dyn Error>> {
        match self
            .0
            .insert_one(doc.clone(), None)
            .await?
            .inserted_id
            .as_object_id()
        {
            Some(post_id) => Ok(post_id.to_hex()),
            None => Err(Box::new(GenerationError)),
        }
    }

    pub async fn get_image(&self, id: &str) -> Result<Option<Document>, mongodb::error::Error> {
        match bson::oid::ObjectId::from_str(id) {
            Ok(x) => self.0.find_one(doc! {"_id": x }, None).await,
            Err(_) => Ok(None),
        }
    }

    pub async fn get_index(&self, page: u64) -> Result<Vec<PostResponse>, mongodb::error::Error> {
        let mut list: Vec<PostResponse> = Vec::new();
        let mut cursor = self
            .0
            .find(
                None,
                FindOptions::builder()
                    .limit(10)
                    .skip(Some((page - 1) * 10))
                    .sort(doc! {"date": -1})
                    .build(),
            )
            .await?;
        while let Some(doc) = cursor.next().await {
            list.push(from_document(doc?)?);
        }
        Ok(list)
    }
}
