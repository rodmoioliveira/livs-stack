use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "titles")]
pub struct Title {
    pub id: Option<i64>,
    pub isbn: i64,
    pub author: String,
    pub title: String,
    pub publisher: String,
    pub year: i16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Data { data }
    }
}
