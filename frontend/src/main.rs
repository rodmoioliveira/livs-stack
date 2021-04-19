use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use frontend::handlers;
use handlebars::Handlebars;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap();
    let localhost = String::from(format!("{}:{}", host, port));
    println!("Server running in {}", localhost);

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(handlers::error::_404())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .app_data(handlebars_ref.clone())
            .service(web::resource("/").route(web::get().to(handlers::root::index)))
            .service(web::resource("/{user}/{data}").route(web::get().to(handlers::titles::user)))
    })
    .bind(localhost)?
    .run()
    .await
}
