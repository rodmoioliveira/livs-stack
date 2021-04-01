use actix_web::{middleware::Logger, App, HttpServer};
use backend::handlers;
use config;
use deadpool_postgres::{self};
use dotenv::dotenv;
use serde::Deserialize;
use tokio_postgres::NoTls;

#[derive(Debug, Deserialize)]
struct Config {
    pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}

// TODO: ver! https://github.com/actix/examples/tree/master/database_interactions/pg/src
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let client = pool.get().await.unwrap();
    let stmt = client.prepare("SELECT * from books").await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let value: String = rows[0].get(2);
    println!("{:#?}", value);

    let localhost = String::from("0.0.0.0:8081");
    println!("Server running in {}", localhost);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(handlers::index)
            .service(handlers::book)
            .service(handlers::books)
    })
    .bind(localhost)?
    .run()
    .await
}
