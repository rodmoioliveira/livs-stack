use crate::{errors, models, queries};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(db_pool: web::Data<Pool>) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::db::Sets = queries::sets::all(&client).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
