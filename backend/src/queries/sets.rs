use crate::{errors, models};
use deadpool_postgres::Client;
use std::collections::HashMap;

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

    let format_hash: HashMap<i16, models::db::SetFormat> =
        format
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.id).or_insert(cur);
                acc
            });

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

    let genre_hash: HashMap<i64, models::db::SetGenre> =
        genre
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.id).or_insert(cur);
                acc
            });

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

    let language_hash: HashMap<i64, models::db::SetLanguage> =
        language
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.id).or_insert(cur);
                acc
            });

    Ok(models::db::Sets {
        language: language_hash,
        format: format_hash,
        genre: genre_hash,
    })
}
