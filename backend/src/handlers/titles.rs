use crate::{errors, models, queries, querystrings, utils};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(
    web::Query(order_by_qs): web::Query<querystrings::core::Order>,
    web::Query(filter_qs): web::Query<querystrings::titles::Filters>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let (result, count) =
        queries::titles::all(&client, order_by_qs.clone(), filter_qs.clone()).await?;
    let pagination = utils::get_pagination(order_by_qs, count, result.len() as i64)?;

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
    let result: models::db::Title = queries::titles::one(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn add(
    title: web::Json<models::db::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::db::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::titles::add(&client, title_info).await?;

    Ok(HttpResponse::Created().json(models::response::Data::new(result)))
}

pub async fn update(
    web::Path(id): web::Path<i64>,
    title: web::Json<models::db::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::db::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::titles::update(&client, id, title_info).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn delete(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::titles::delete(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
