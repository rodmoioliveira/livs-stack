use crate::{errors, models, queries, querystrings};
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

    let after_id = filter_qs.after_id.unwrap_or(0);
    let limit = order_by_qs.limit.unwrap_or(count);
    let items_current = if count < limit { count } else { limit };
    let items_total = count + after_id;
    let page_total = (items_total as f64 / limit as f64).ceil() as i64;
    let page_current = (after_id / limit) + 1;
    let has_next = page_current < page_total;
    let has_prev = page_current > 1;

    // assert!(after_id % limit == 0);

    // TODO: make it work with filters

    // select id, row_number() over(), count(*) over() as count from titles where language = 7;
    // Using ROW_NUMBER() function for pagination
    // https://www.postgresqltutorial.com/postgresql-row_number/

    // http://localhost:8081/titles?genres=1,2,3,4,5,6,10,20&formats=1,2,3,4&languages=1,2,3,4,5&order_by=id&limit=5
    // http://localhost:8081/titles?genres=1,2,3,4,5,6,10,20&formats=1,2,3,4&languages=1,2,3,4,5&order_by=id&limit=5&after_id=76
    // http://localhost:8081/titles?genres=1,2,3,4,5,6,10,20&formats=1,2,3,4&languages=1,2,3,4,5&order_by=id&limit=5&after_id=135
    // you still can use after_id!!!

    // GET PAGINATION
    let pagination = models::db::Pagination {
        page_current,
        items_current,
        page_total,
        items_total,
        has_prev,
        has_next,
        limit,
    };

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
