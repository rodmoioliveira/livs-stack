use crate::{errors, models, queries, querystrings, utils};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(
    web::Query(order_by_qs): web::Query<querystrings::Order>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let (result, count) = queries::languages::all(&client, &order_by_qs).await?;
    let pagination = utils::get_pagination(&order_by_qs, count, result.len() as i64)?;

    Ok(
        HttpResponse::Ok().json(models::response::DataWithPagination::new(
            result, pagination,
        )),
    )
}

pub async fn one(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::db::Language = queries::languages::one(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn add(
    language: web::Json<models::db::Language>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let language_info: models::db::Language = language.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::languages::add(&client, language_info).await?;

    Ok(HttpResponse::Created().json(models::response::Data::new(result)))
}

pub async fn update(
    web::Path(id): web::Path<i64>,
    language: web::Json<models::db::Language>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let language_info: models::db::Language = language.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::languages::update(&client, id, language_info).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn delete(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::languages::delete(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
