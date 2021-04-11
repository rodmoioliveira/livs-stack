use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use backend::handlers::{root, titles};
use backend::{db, errors};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = db::Config::from_env().unwrap();
    let db_pool = config.pg.create_pool(NoTls).unwrap();

    let localhost = String::from("0.0.0.0:8081");
    println!("Server running in {}", localhost);

    // https://actix.rs/actix-web/actix_web/web/struct.JsonConfig.html#method.error_handler
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

    // https://docs.rs/actix-web/3.3.2/actix_web/web/struct.PathConfig.html
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
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(root::index)
            .service(titles::add_title)
            .service(titles::delete_title)
            .service(titles::get_title)
            .service(titles::get_titles)
            .service(titles::update_title)
    })
    .bind(localhost)?
    .run()
    .await
}
