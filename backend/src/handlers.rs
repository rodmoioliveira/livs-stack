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

#[get("/titles/{isbn}")]
pub async fn get_title(
    web::Path(isbn): web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result: models::Title = db::get_title(&client, isbn).await?;

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

#[put("/titles/{isbn}")]
pub async fn update_title(
    web::Path(isbn): web::Path<String>,
    title: web::Json<models::Title>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let title_info: models::Title = title.into_inner();
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::update_title(&client, isbn, title_info).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}

#[delete("/titles/{isbn}")]
pub async fn delete_title(
    web::Path(isbn): web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, errors::MyError> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let result = db::delete_title(&client, isbn).await?;

    Ok(HttpResponse::Ok().json(models::Data::new(result)))
}
