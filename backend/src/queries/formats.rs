use crate::{errors, models, querystrings};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(
    client: &Client,
    order_by_qs: querystrings::Order,
) -> Result<(Vec<models::db::Format>, i64), errors::MyError> {
    let _stmt = include_str!("../sql/formats/all.sql");
    let _stmt = _stmt.replace("$order_by", &order_by_qs.to_sql());
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

    match count {
        0 => Ok((vec![], count)),
        _ => {
            let result: Vec<models::db::Format> =
                serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

            Ok((result, count))
        }
    }
}

pub async fn one(
    client: &Client,
    id: i16,
) -> Result<models::db::Format, errors::MyError> {
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
    format: models::db::Format,
) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/add.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Format::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&format.format])
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
    format: models::db::Format,
) -> Result<models::db::Format, errors::MyError> {
    let _stmt = include_str!("../sql/formats/update.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Format::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&format.format, &id])
        .await?
        .iter()
        .map(|row| models::db::Format::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Format>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(
    client: &Client,
    id: i16,
) -> Result<models::db::Format, errors::MyError> {
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
