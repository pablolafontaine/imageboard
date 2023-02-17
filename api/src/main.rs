mod db;
mod upload;
mod view;
use actix_cors::Cors;
use actix_web::{
    http, middleware,
    web::{self, Data},
    App, HttpServer,
};
    pub static CDN_URL: &str = std::env!("CDN_URL");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let max_file_size: usize = std::env!("MAX_FILE_SIZE")
        .parse::<usize>()
        .ok()
        .unwrap_or(3078000);
    const API_PORT: &str = std::env!("API_PORT");
    const UI_PORT: &str = std::env!("TRUNK_SERVE_PORT");
    const UI_HOST: &str = std::env!("TRUNK_SERVE_HOST");
    let database = db::Db::new(&std::fs::read_to_string("/run/secrets/mongodb_key")?.trim(), std::fs::read_to_string("/run/secrets/cdn_access_key")?.trim().to_owned()).await;
    match database {
        Ok(d) => {
            HttpServer::new(move || {
                let cors = Cors::default()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().ends_with(format!("{}:{}", UI_HOST, UI_PORT).as_bytes())
                    })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600);
                App::new()
                    .wrap(cors)
                    .app_data(awmp::PartsConfig::default().with_file_limit(max_file_size))
                    .wrap(middleware::Logger::default())
                    .app_data(Data::new(d.clone()))
                    .route("/upload", web::post().to(upload::upload_image))
                    .route("/", web::get().to(view::fetch_index_page))
                    .service(
                        web::resource("/{page:[1-9]+}")
                            .route(web::get().to(view::fetch_index_page)),
                    )
                    .service(view::view_image)
                    .default_service(web::route().to(view::not_found))
            })
            .bind(format!("0.0.0.0:{}", API_PORT))?
            .run()
            .await
        }
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            e.to_string(),
        )),
    }
}
