use crate::{errors, models, queries, querystrings};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(
    web::Query(order_by_qs): web::Query<querystrings::core::Order>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: Vec<models::db::Format> = queries::formats::all(&client, order_by_qs).await?;

    // GET PAGINATION
    let pagination = models::db::Pagination {
        total_count: 1,
        page_count: 1,
        total_pages: 1,
        current_page: 1,
        has_prev: true,
        has_next: true,
    };

    Ok(
        HttpResponse::Ok().json(models::response::DataWithPagination::new(
            result, pagination,
        )),
    )
}

pub async fn one(
    web::Path(id): web::Path<i16>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::db::Format = queries::formats::one(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn add(
    format: web::Json<models::db::Format>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let format_info: models::db::Format = format.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::formats::add(&client, format_info).await?;

    Ok(HttpResponse::Created().json(models::response::Data::new(result)))
}

pub async fn update(
    web::Path(id): web::Path<i16>,
    format: web::Json<models::db::Format>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let format_info: models::db::Format = format.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::formats::update(&client, id, format_info).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}

pub async fn delete(
    web::Path(id): web::Path<i16>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = queries::formats::delete(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::response::Data::new(result)))
}
