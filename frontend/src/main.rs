use actix_web::middleware;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use frontend::{
    handlers::{error, formats, genres, languages, root, titles},
    models,
};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use std::{env, io};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let localhost = env::var("ENDPOINT_FRONTEND").unwrap();
    println!("Server running in {}", localhost);

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let client = Client::builder().build().unwrap();
    let endpoints = models::Endpoints::new();

    HttpServer::new(move || {
        App::new()
            .wrap(error::_404())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .data(client.clone())
            .data(endpoints.clone())
            .app_data(handlebars_ref.clone())
            .service(web::resource("/").route(web::get().to(root::index)))
            .service(web::resource("/formats").route(web::get().to(formats::all)))
            .service(web::resource("/genres").route(web::get().to(genres::all)))
            .service(web::resource("/languages").route(web::get().to(languages::all)))
            .service(web::resource("/titles").route(web::get().to(titles::all)))
    })
    .bind(localhost)?
    .run()
    .await
}
