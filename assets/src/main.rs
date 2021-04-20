use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
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
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").show_files_listing())
    })
    .bind(localhost)?
    .run()
    .await
}
