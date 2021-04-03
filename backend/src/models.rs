use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "titles")]
pub struct Title {
    pub isbn: i64,
    pub author: String,
    pub title: String,
    pub editor: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}
