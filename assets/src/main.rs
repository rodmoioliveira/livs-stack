use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http::header, middleware, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let localhost = env::var("ENDPOINT_ASSETS").unwrap();
    println!("Server running in {}", localhost);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin())
            .wrap(
                middleware::DefaultHeaders::new().header(header::CACHE_CONTROL, "max-age=31536000"),
            )
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(Files::new("/static", "static/").show_files_listing())
    })
    .bind(localhost)?
    .run()
    .await
}
