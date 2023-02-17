use bson::from_document;
use crate::CDN_URL;
use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, Document},
    options::FindOptions, Collection,
};
use std::{error::Error, str::FromStr};
use types::{GenerationError, PostResponse};


#[derive(Clone)]
pub struct Db{
    collection: Collection<Document>,
    pub cdn_access_key: String,
    pub reqwest_client: reqwest::Client,
}

impl Db {
    pub async fn new(mongo_uri: &str, cdn_access_key: String) -> Result<Self, Box<dyn Error>> {
        let mongo_client = mongodb::Client::with_uri_str(mongo_uri).await?;
        let reqwest_client = reqwest::Client::new();
        Ok(Db { collection: mongo_client.database("ImageBoard").collection("Images"), cdn_access_key, reqwest_client } )
    }
    pub async fn add_image(&self, doc: &Document) -> Result<String, Box<dyn Error>> {
        match self
            .collection
            .insert_one(doc.clone(), None)
            .await?
            .inserted_id
            .as_object_id()
        {
            Some(post_id) => Ok(post_id.to_hex()),
            None => Err(Box::new(GenerationError)),
        }
    }

    pub async fn cdn_upload(&self, file_name: &str, file: awmp::File) -> Result<(), Box<dyn Error>>{
            let data = std::fs::read(file.into_inner().into_temp_path().to_path_buf())?;
            self.reqwest_client.put(format!("{}/{}",CDN_URL, file_name)).header("AccessKey", &self.cdn_access_key).header("content-type", "application/octet_stream").body(data).send().await?;
            Ok(())
    }


    pub async fn get_image(&self, id: &str) -> Result<Option<Document>, mongodb::error::Error> {
        match bson::oid::ObjectId::from_str(id) {
            Ok(x) => self.collection.find_one(doc! {"_id": x }, None).await,
            Err(_) => Ok(None),
        }
    }

    pub async fn get_index(&self, page: u64) -> Result<Vec<PostResponse>, mongodb::error::Error> {
        let mut list: Vec<PostResponse> = Vec::new();
        let mut cursor = self
            .collection
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
