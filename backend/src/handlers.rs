use crate::{db, errors, models};
use actix_web::{delete, get, post, put, web, HttpResponse, Result};
use deadpool_postgres::{Client, Pool};

#[get("/")]
pub async fn index() -> Result<String, errors::MyError> {
    Ok(format!("index"))
}

#[get("/titles")]
pub async fn get_titles(db_pool: web::Data<Pool>) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: Vec<models::Title> = db::get_titles(&client).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

#[get("/titles/{id}")]
pub async fn get_title(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::Title = db::get_title(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

#[post("/titles")]
pub async fn add_title(
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::insert_title(&client, title_info).await?;

    Ok(HttpResponse::Created().json(models::Data::new(result)))
}

#[put("/titles/{id}")]
pub async fn update_title(
    web::Path(id): web::Path<i64>,
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::update_title(&client, id, title_info).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

#[delete("/titles/{id}")]
pub async fn delete_title(
    web::Path(id): web::Path<i64>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::delete_title(&client, id).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}
