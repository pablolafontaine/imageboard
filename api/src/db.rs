use bson::from_document;
use futures::stream::StreamExt;
use aws_sdk_s3::{config, Credentials, Region, types::ByteStream, output::PutObjectOutput};
use mongodb::{
    bson::{doc, Document},
    options::FindOptions, Collection,
};
use std::{error::Error, str::FromStr};
use types::{GenerationError, PostResponse};

#[derive(Clone)]
pub struct Db{
    collection: Collection<Document>,
    s3_client: aws_sdk_s3::Client,
}

impl Db {
    pub async fn new(mongo_uri: &str, aws_access_key: &str, aws_secret_access_key: &str, region: &str) -> Result<Self, Box<dyn Error>> {
        let mongo_client = mongodb::Client::with_uri_str(mongo_uri).await?;

        let cred = Credentials::new(aws_access_key, aws_secret_access_key, None, None, "loaded-from-custom-env");
        let aws_config = config::Builder::new().region(Region::new(region.to_string())).credentials_provider(cred).build();
        let aws_client = aws_sdk_s3::Client::from_conf(aws_config);
        Ok(Db { collection: mongo_client.database("ImageBoard").collection("Images"), s3_client: aws_client } )
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

    pub async fn upload_image_s3(&self, body: ByteStream, key: &str) -> Result<PutObjectOutput, Box<dyn Error>>{
        let req = self.s3_client.put_object().bucket("kouchan").key(key).body(body).content_type(mime_guess::from_path(key).first_or_octet_stream().to_string());
        Ok(req.send().await?)
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
