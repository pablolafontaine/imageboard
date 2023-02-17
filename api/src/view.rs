use crate::db::Db;
use actix_web::{
    get,
    web::{self, Json},
    HttpResponse, 
};
use std::error::Error;
use types::{PostLoadError, PostResponse};

#[get("/post/{id}")]
pub async fn view_image(
    db: web::Data<Db>,
    id: web::Path<String>,
) -> Result<Json<String>, Box<dyn Error>> {
    if let Some(doc) = db.get_image(&id).await.ok() {
        match doc {
            Some(post) => Ok(Json(serde_json::to_string(&post)?)),
            None => Err(Box::new(PostLoadError)),
        }
    }
    else{
         Err(Box::new(PostLoadError))
        }
    }
pub async fn fetch_index_page(
    db: web::Data<Db>,
    page: Option<web::Path<u64>>,
) -> Result<Json<Vec<PostResponse>>, Box<dyn Error>> {
    let index = if let Some(p) = page {
        db.get_index(p.into_inner()).await
    } else {
        db.get_index(1).await
    }?;

    Ok(Json(index))
}

pub async fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body("404")
}
