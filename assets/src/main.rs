use actix_files::Files;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let localhost = String::from("0.0.0.0:8082");
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
