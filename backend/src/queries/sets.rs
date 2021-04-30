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
    let format: Vec<models::db::Set> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    let format_hash: HashMap<String, models::db::SetVec> =
        format
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.format.clone()).or_insert(cur.to_vec());
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
    let genre: Vec<models::db::Set> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    let genre_hash: HashMap<String, models::db::SetVec> =
        genre
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.genre.clone()).or_insert(cur.to_vec());
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
    let language: Vec<models::db::Set> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    let language_hash: HashMap<String, models::db::SetVec> =
        language
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, cur| {
                acc.entry(cur.language.clone()).or_insert(cur.to_vec());
                acc
            });

    Ok(models::db::Sets {
        language: language_hash,
        format: format_hash,
        genre: genre_hash,
    })
}
