use actix_cors::Cors;
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
        let cors = Cors::default().allow_any_origin().max_age(3600);
        // https://stackoverflow.com/questions/65863107/how-to-set-expire-or-cache-control-header-when-serving-static-files-with-act
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").show_files_listing())
    })
    .bind(localhost)?
    .run()
    .await
}
