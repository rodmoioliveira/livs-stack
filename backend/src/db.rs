use crate::{errors, models};
use config;
use deadpool_postgres::Client;
use serde::Deserialize;
use tokio_pg_mapper::FromTokioPostgresRow;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}

pub async fn get_titles(client: &Client) -> Result<Vec<models::Title>, errors::MyError> {
    let _stmt = include_str!("./sql/get_titles.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(result)
}

pub async fn get_title(client: &Client, isbn: i64) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("./sql/get_title.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&isbn])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn insert_title(
    client: &Client,
    title: models::Title,
) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("./sql/insert_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[
                &title.isbn,
                &title.author,
                &title.title,
                &title.publisher,
                &title.year,
            ],
        )
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update_title(
    client: &Client,
    isbn: i64,
    title: models::Title,
) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("./sql/update_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[
                &title.isbn,
                &title.author,
                &title.title,
                &title.publisher,
                &title.year,
                &isbn,
            ],
        )
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete_title(client: &Client, isbn: i64) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("./sql/delete_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&isbn])
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
