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
