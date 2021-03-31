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

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let localhost = String::from("0.0.0.0:8081");
    println!("Server running in {}", localhost);

    for i in 1..10 {
        let client = pool.get().await.unwrap();
        let stmt = client.prepare("SELECT 1 + $1").await.unwrap();
        let rows = client.query(&stmt, &[&i]).await.unwrap();
        let value: i32 = rows[0].get(0);
        println!("{}", value);
        assert_eq!(value, i + 1);
    }

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
