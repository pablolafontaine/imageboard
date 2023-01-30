mod db;
mod upload;
mod view;
use actix_web::{
    middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer,
};


async fn form() -> HttpResponse {
    let html = r#"
        <form action="/upload" method="post" enctype="multipart/form-data">
            <input type="text" name="title" placeholder="Title (Max: 100 Characters)" required maxlength="100">
            <input type="text" name="text" placeholder="Description (Max: 2000 Characters)" required maxlength="2000">
            <input type="file" name="file" accept="image/png, image/jpeg" required>
            <input type="submit" value="Upload">
        </form>
    "#;
    HttpResponse::Ok().content_type("text/html").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./uploads")?;
    let max_file_size: usize = std::env!("MAX_FILE_SIZE").parse::<usize>().ok().unwrap_or(3078000);
    const API_PORT: &str = std::env!("API_PORT");
    const UI_PORT: &str = std::env!("TRUNK_SERVE_PORT");
    const UI_HOST: &str = std::env!("TRUNK_SERVE_HOST"); 
            let database = db::Db::new(&std::fs::read_to_string("/run/secrets/mongodb_key")?.trim()).await;
            match database {
                Ok(d) => {
                    HttpServer::new(move || {
                        App::new()
                            .app_data(awmp::PartsConfig::default().with_file_limit(max_file_size))
                            .wrap(middleware::Logger::default())
                            .app_data(Data::new(d.clone()))
                            .route("/post", web::get().to(form))
                            .route("/upload", web::post().to(upload::upload_image))
                            .route("/", web::get().to(view::fetch_index_page))
                            .service(web::resource("/{page:[1-9]+}").route(web::get().to(view::fetch_index_page)))
                            .service(view::view_image)
                            .service(view::fetch_image)
                            .default_service(web::route().to(view::not_found))
                    })
                    .bind(format!("0.0.0.0:{}", API_PORT))?
                    .run()
                    .await
                }
                Err(e) => {
                    Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))
                }
            }
            
    }
