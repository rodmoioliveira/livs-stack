use actix_web::{middleware::Logger, App, HttpServer};
use backend::{db, handlers};
use dotenv::dotenv;
use tokio_postgres::NoTls;

// TODO: ver! https://github.com/actix/examples/tree/master/database_interactions/pg/src
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
            .wrap(Logger::default())
            .service(handlers::index)
            .service(handlers::title)
            .service(handlers::titles)
    })
    .bind(localhost)?
    .run()
    .await
}
