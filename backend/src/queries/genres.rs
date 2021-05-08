use crate::{errors, models, querystrings, utils};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(
    client: &Client,
    order_by_qs: querystrings::Order,
) -> Result<(Vec<models::db::Genre>, i64), errors::MyError> {
    let _stmt = include_str!("../sql/genres/all.sql");
    let _stmt = _stmt.replace("$order_by", &order_by_qs.clone().to_sql());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let counts: Vec<models::db::Count> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;
    let count = counts
        .first()
        .unwrap_or(&models::db::Count { count: 0 })
        .count;

    let limit = order_by_qs.limit.unwrap_or(count);
    if count == 0 || limit == 0 {
        return Ok((vec![], count));
    };

    utils::handle_bad_pagination(count, &order_by_qs)?;
    utils::handle_bad_offset(count, &order_by_qs)?;

    let result: Vec<models::db::Genre> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;
    Ok((result, count))
}

pub async fn one(
    client: &Client,
    id: i64,
) -> Result<models::db::Genre, errors::MyError> {
    let _stmt = include_str!("../sql/genres/one.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::db::Genre> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn add(
    client: &Client,
    genre: models::db::Genre,
) -> Result<models::db::Genre, errors::MyError> {
    let _stmt = include_str!("../sql/genres/add.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Genre::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&genre.genre])
        .await?
        .iter()
        .map(|row| models::db::Genre::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Genre>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update(
    client: &Client,
    id: i64,
    genre: models::db::Genre,
) -> Result<models::db::Genre, errors::MyError> {
    let _stmt = include_str!("../sql/genres/update.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Genre::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&genre.genre, &id])
        .await?
        .iter()
        .map(|row| models::db::Genre::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Genre>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(
    client: &Client,
    id: i64,
) -> Result<models::db::Genre, errors::MyError> {
    let _stmt = include_str!("../sql/genres/delete.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Genre::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| models::db::Genre::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Genre>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
