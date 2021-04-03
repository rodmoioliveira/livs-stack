use crate::{errors, models};
use actix_web::{error, get, web, Error, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

#[get("/")]
async fn index() -> Result<String> {
    Ok(format!("index"))
}

#[get("/titles")]
async fn titles(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let query: String = String::from("SELECT * FROM titles");
    let stmt = client.prepare(&query).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();

    let result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(HttpResponse::Ok().json(models::Data { data: result }))
}

#[get("/titles/{isbn}")]
async fn title(
    web::Path(isbn): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let query: String = format!("SELECT * FROM titles WHERE isbn = '{}'", isbn);
    let stmt = client.prepare(&query).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();

    let result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    match result.len() {
        1 => Ok(HttpResponse::Ok().json(models::Data {
            data: result.first(),
        })),
        _ => Err(error::ErrorNotFound("isbn not found")),
    }
}
