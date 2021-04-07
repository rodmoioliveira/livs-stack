use crate::{db, errors, models};
use actix_web::{error, get, post, web, Error, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn index() -> Result<String> {
    Ok(format!("index"))
}

#[get("/titles")]
pub async fn get_titles(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let query: String = String::from("SELECT * FROM titles");
    let stmt = client.prepare(&query).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();

    let result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(HttpResponse::Ok().json(models::Data { data: result }))
}

#[get("/titles/{isbn}")]
pub async fn get_title(
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

#[post("/titles")]
pub async fn add_title(
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let new_title = db::insert_title(&client, title_info).await?;

    Ok(HttpResponse::Ok().json(new_title))
}
