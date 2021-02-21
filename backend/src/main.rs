use actix_web::{middleware::Logger, App, HttpServer};
use backend::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let localhost = String::from("0.0.0.0:8081");
    println!("Server running in {}", localhost);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(handlers::index)
            .service(handlers::book)
            .service(handlers::books)
    })
    .bind(localhost)?
    .run()
    .await
}
