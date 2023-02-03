use crate::db::Db;
use actix_web::{web, HttpResponse};
use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};
use types::{ContentLengthError, FileSizeError, SplitFileExtError};
use uuid::Uuid;

pub async fn generate_image_id(
    db: web::Data<Db>,
    file_data: awmp::File,
    title: &str,
    text: &str,
) -> Result<String, Box<dyn Error>> {
    if let Some(extension) = file_data.sanitized_file_name().rsplit_once('.') {
        let filepath = format!("./uploads/{}.{}", Uuid::new_v4(), extension.1);
        file_data
            .into_inner()
            .persist(filepath.as_str())
            .map(|_| filepath.as_str())?;

        let doc = bson::doc! {
            "img_path": filepath.as_str(),
            "title": title,
            "text": text,
            "date": SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
            "comments" : []
        };

        db.add_image(&doc).await
    } else {
        Err(Box::new(SplitFileExtError))
    }
}

pub async fn upload_image(
    db: web::Data<Db>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse, Box<dyn Error>> {
    let hm = parts.texts.as_hash_map();
    if let (Some(file), Some(title), Some(text)) = (
        parts.files.take("file").pop(),
        hm.get("title"),
        hm.get("text"),
    ) {
        let max_title_len: usize = std::env!("MAX_TITLE_LEN")
            .parse::<usize>()
            .ok()
            .unwrap_or(100);
        let max_body_len: usize = std::env!("MAX_BODY_LEN")
            .parse::<usize>()
            .ok()
            .unwrap_or(2000);

        if title.is_empty()
            || text.is_empty()
            || title.len() > max_body_len
            || text.len() > max_title_len
        {
            return Err(Box::new(ContentLengthError));
        }

        Ok(HttpResponse::Found()
            .append_header((
                "Location",
                format!("/post/{}", generate_image_id(db, file, title, text).await?),
            ))
            .finish())
    } else {
        Err(Box::new(FileSizeError))
    }
}
