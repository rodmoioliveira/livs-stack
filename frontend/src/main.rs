use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use frontend::{handlers, models};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let localhost = env::var("ENDPOINT_FRONTEND").unwrap();
    println!("Server running in {}", localhost);

    let endpoints = models::types::Endpoints::new();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let client = Client::builder().build().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(handlers::error::_404())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(client.clone())
            .data(endpoints.clone())
            .app_data(handlebars_ref.clone())
            .service(web::resource("/").route(web::get().to(handlers::root::index)))
    })
    .bind(localhost)?
    .run()
    .await
}
