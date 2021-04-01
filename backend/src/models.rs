use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "books")]
pub struct Book {
    pub id: i64,
    pub isbn: String,
    pub author: String,
    pub title: String,
    pub editor: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Books {
    pub data: Vec<Book>,
}
