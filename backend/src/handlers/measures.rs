use crate::{errors, models, queries, querystrings, utils};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(
    web::Query(order_by_qs): web::Query<querystrings::Order>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let (result, count) = queries::measures::all(&client, &order_by_qs).await?;
    let pagination = utils::get_pagination(&order_by_qs, count, result.len() as i64)?;

    Ok(
        HttpResponse::Ok().json(models::response::DataWithPagination::new(
            result, pagination,
        )),
    )
}

pub async fn one(
    web::Path(title_id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::db::Measure = queries::measures::one(&client, title_id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn add(
    measure: web::Json<models::db::Measure>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let measure_info: models::db::Measure = measure.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::measures::add(&client, measure_info).await?;

    Ok(HttpResponse::Created().json(models::response::Data::new(result)))
}

pub async fn update(
    web::Path(title_id): web::Path<i64>,
    measure: web::Json<models::db::Measure>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let measure_info: models::db::Measure = measure.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::measures::update(&client, title_id, measure_info).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn delete(
    web::Path(title_id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::measures::delete(&client, title_id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
