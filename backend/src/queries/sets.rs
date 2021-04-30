use crate::{errors, models};
use deadpool_postgres::Client;

pub async fn all(client: &Client) -> Result<models::db::Sets, errors::MyError> {
    let _stmt = include_str!("../sql/sets/format.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let format: Vec<models::db::SetFormat> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    let _stmt = include_str!("../sql/sets/genre.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let genre: Vec<models::db::SetGenre> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    let _stmt = include_str!("../sql/sets/language.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let language: Vec<models::db::SetLanguage> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(models::db::Sets {
        language,
        format,
        genre,
    })
}
