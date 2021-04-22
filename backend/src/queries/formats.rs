use crate::{errors, models, querystrings};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(
    client: &Client,
    order_by_qs: querystrings::titles::Order,
    filter_qs: querystrings::titles::Filters,
) -> Result<Vec<models::db::Format>, errors::MyError> {
    let _stmt = include_str!("../sql/formats/all.sql");
    let _stmt = _stmt.replace("$order_by", &order_by_qs.to_sql());
    let _stmt = _stmt.replace("$filters", &filter_qs.to_sql());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let result: Vec<models::db::Format> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(result)
}

pub async fn one(client: &Client, id: i16) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/one.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::db::Format> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn add(
    client: &Client,
    title: models::db::Format,
) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/add.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Format::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&title.format])
        .await?
        .iter()
        .map(|row| models::db::Format::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Format>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update(
    client: &Client,
    id: i16,
    title: models::db::Format,
) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/update.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Format::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&title.format, &id])
        .await?
        .iter()
        .map(|row| models::db::Format::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Format>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(client: &Client, id: i16) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/delete.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Format::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| models::db::Format::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Format>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
