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
