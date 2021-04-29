use crate::{errors, models, queries};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(db_pool: web::Data<Pool>) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: Vec<models::db::SetFormat> = queries::sets_formats::all(&client).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn one(
    web::Path(id): web::Path<i16>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::db::SetFormat = queries::sets_formats::one(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
