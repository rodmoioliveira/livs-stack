use actix_web::{middleware, App, HttpServer};
use backend::{db, handlers};
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

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(handlers::index)
            .service(handlers::get_title)
            .service(handlers::get_titles)
            .service(handlers::add_title)
    })
    .bind(localhost)?
    .run()
    .await
}
