use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use backend::handlers::{formats, root, titles};
use backend::{db, errors};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = db::config::Config::from_env().unwrap();
    let db_pool = config.pg.create_pool(NoTls).unwrap();

    let localhost = env::var("ENDPOINT_BACKEND").unwrap();
    println!("Server running in {}", localhost);

    let query_cfg = web::QueryConfig::default().error_handler(|err, _req| {
        let err_msg = err.to_string();
        error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(errors::JsonError::new(err_msg)),
        )
        .into()
    });

    let json_cfg = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            let err_msg = err.to_string();
            error::InternalError::from_response(
                err,
                HttpResponse::BadRequest().json(errors::JsonError::new(err_msg)),
            )
            .into()
        });

    let path_cfg = web::PathConfig::default().error_handler(|err, _req| {
        let err_msg = err.to_string();
        error::InternalError::from_response(
            err,
            HttpResponse::BadRequest().json(errors::JsonError::new(err_msg)),
        )
        .into()
    });

    HttpServer::new(move || {
        App::new()
            .app_data(json_cfg.clone())
            .app_data(path_cfg.clone())
            .app_data(query_cfg.clone())
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(web::resource("/").route(web::get().to(root::index)))
            .service(
                web::scope("/titles")
                    .service(
                        web::resource("")
                            .route(web::get().to(titles::all))
                            .route(web::post().to(titles::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(titles::one))
                            .route(web::delete().to(titles::delete))
                            .route(web::put().to(titles::update)),
                    ),
            )
            .service(
                web::scope("/formats")
                    .service(
                        web::resource("")
                            .route(web::get().to(formats::all))
                            .route(web::post().to(formats::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(formats::one))
                            .route(web::delete().to(formats::delete))
                            .route(web::put().to(formats::update)),
                    ),
            )
    })
    .bind(localhost)?
    .run()
    .await
}
