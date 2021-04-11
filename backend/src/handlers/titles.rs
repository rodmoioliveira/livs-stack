use crate::{db, errors, models};
use actix_web::{web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

pub async fn all(db_pool: web::Data<Pool>) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: Vec<models::Title> = db::get_titles(&client).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

pub async fn one(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::Title = db::get_title(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

pub async fn add(
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::insert_title(&client, title_info).await?;

    Ok(HttpResponse::Created().json(models::Data::new(result)))
}

pub async fn update(
    web::Path(id): web::Path<i64>,
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::update_title(&client, id, title_info).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

pub async fn delete(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::delete_title(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}
