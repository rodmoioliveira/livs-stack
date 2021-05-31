use actix_web::{error, middleware, web, App, HttpResponse, HttpServer};
use backend::handlers::{
    authors, customers, formats, genres, inventory, languages, measures, publishers, reviews, root,
    titles,
};
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
            .service(
                web::scope("/genres")
                    .service(
                        web::resource("")
                            .route(web::get().to(genres::all))
                            .route(web::post().to(genres::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(genres::one))
                            .route(web::delete().to(genres::delete))
                            .route(web::put().to(genres::update)),
                    ),
            )
            .service(
                web::scope("/publishers")
                    .service(
                        web::resource("")
                            .route(web::get().to(publishers::all))
                            .route(web::post().to(publishers::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(publishers::one))
                            .route(web::delete().to(publishers::delete))
                            .route(web::put().to(publishers::update)),
                    ),
            )
            .service(
                web::scope("/authors")
                    .service(
                        web::resource("")
                            .route(web::get().to(authors::all))
                            .route(web::post().to(authors::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(authors::one))
                            .route(web::delete().to(authors::delete))
                            .route(web::put().to(authors::update)),
                    ),
            )
            .service(
                web::scope("/customers")
                    .service(
                        web::resource("")
                            .route(web::get().to(customers::all))
                            .route(web::post().to(customers::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(customers::one))
                            .route(web::delete().to(customers::delete))
                            .route(web::put().to(customers::update)),
                    ),
            )
            .service(
                web::scope("/reviews")
                    .service(
                        web::resource("")
                            .route(web::get().to(reviews::all))
                            .route(web::post().to(reviews::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(reviews::one))
                            .route(web::delete().to(reviews::delete))
                            .route(web::put().to(reviews::update)),
                    ),
            )
            .service(
                web::scope("/languages")
                    .service(
                        web::resource("")
                            .route(web::get().to(languages::all))
                            .route(web::post().to(languages::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(languages::one))
                            .route(web::delete().to(languages::delete))
                            .route(web::put().to(languages::update)),
                    ),
            )
            .service(
                web::scope("/inventory")
                    .service(
                        web::resource("")
                            .route(web::get().to(inventory::all))
                            .route(web::post().to(inventory::add)),
                    )
                    .service(
                        web::resource("/{id}")
                            .route(web::get().to(inventory::one))
                            .route(web::delete().to(inventory::delete))
                            .route(web::put().to(inventory::update)),
                    ),
            )
            .service(
                web::scope("/measures")
                    .service(
                        web::resource("")
                            .route(web::get().to(measures::all))
                            .route(web::post().to(measures::add)),
                    )
                    .service(
                        web::resource("/{title_id}")
                            .route(web::get().to(measures::one))
                            .route(web::delete().to(measures::delete))
                            .route(web::put().to(measures::update)),
                    ),
            )
    })
    .bind(localhost)?
    .run()
    .await
}
