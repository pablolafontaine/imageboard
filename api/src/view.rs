use std::error::Error;
use crate::db::Db;
use actix_files::NamedFile;
use actix_web::{get, web::{self, Json}, HttpResponse};
use std::fmt;
use types::PostResponse;

#[derive(Debug)]
struct PostLoadError;
impl Error for PostLoadError{}
impl fmt::Display for PostLoadError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error loading post!")
    }
}

//{"_id":{"$oid":"63d1edd540dd6211bb9186d2"},"img_path":"./uploads/3f625449-52ae-42af-92ae-a0a3e9a64958.jpeg","title":"gdsgds","text":"gdsgds","date":1674702293,"comments":[]}
#[get("/post/{id}")]
pub async fn view_image(db: web::Data<Db>, id: web::Path<String>) -> Result<Json<PostResponse>, Box<dyn Error>> {
    match db.get_image(&id).await {
        Ok(doc) => match doc {
            Some(post) => { 
                        Ok(Json(PostResponse::new( 
                            post.get("title").unwrap().as_str().unwrap().to_string(),  
                            post.get("text").unwrap().as_str().unwrap().to_string(),
                            post.get("date").unwrap().as_i64().unwrap(),
                            post.get("img_path").unwrap().as_str().unwrap().to_string(),
                            ).await
                        ))
            }
            None => Err(Box::new(PostLoadError)),
        },
        Err(_) => Err(Box::new(PostLoadError)),
    }
}

#[get("/uploads/{path}")]
async fn fetch_image(path: web::Path<String>) -> Result<NamedFile, std::io::Error> {
    NamedFile::open(format!("./uploads/{}", path.as_str()))
}

pub async fn fetch_index_page(
    db: web::Data<Db>,
    page: Option<web::Path<u64>>,
) -> Result<HttpResponse, Box<dyn Error>> {
    let index = if let Some(p) = page {
        db.get_index(p.into_inner()).await
    } else {
        db.get_index(1).await
    }?;
        Ok(HttpResponse::Ok().content_type("text/html").body(index.into_keys().map(|x| format!("<h1> {} </h1><br>", x)).collect::<Vec<String>>().join(" ")))

}

pub async fn not_found() -> HttpResponse{
    HttpResponse::NotFound().content_type("text/html").body("404: Page not found :[")
}
