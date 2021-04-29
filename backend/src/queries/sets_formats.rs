use crate::{errors, models};
use deadpool_postgres::Client;

pub async fn all(client: &Client) -> Result<Vec<models::db::SetFormat>, errors::MyError> {
    let _stmt = include_str!("../sql/sets_formats/all.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let result: Vec<models::db::SetFormat> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(result)
}

pub async fn one(
    client: &Client,
    id: i16,
) -> Result<models::db::SetFormat, errors::MyError> {
    let _stmt = include_str!("../sql/sets_formats/one.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::db::SetFormat> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}
