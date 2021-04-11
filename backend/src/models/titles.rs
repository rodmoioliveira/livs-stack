use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "titles")]
pub struct Title {
    pub id: Option<i64>,
    pub isbn: String,
    pub author: i64,
    pub edition: i16,
    pub format: String,
    pub language: i64,
    pub genre: i64,
    pub pages: i16,
    pub publisher: i64,
    pub summary: String,
    pub title: String,
    pub year: i16,
}
