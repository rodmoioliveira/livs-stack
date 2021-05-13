use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "titles")]
pub struct Title {
    pub id: Option<i64>,
    pub isbn: String,
    pub author: i64,
    pub edition: i16,
    pub format: i16,
    pub language: i64,
    pub genre: i64,
    pub pages: i16,
    pub publisher: i64,
    pub summary: String,
    pub title: String,
    pub year: i16,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "formats")]
pub struct Format {
    pub id: Option<i16>,
    pub format: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "genres")]
pub struct Genre {
    pub id: Option<i64>,
    pub genre: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "languages")]
pub struct Language {
    pub id: Option<i64>,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
    pub items_current: i64,
    pub items_total: i64,
    pub limit: i64,
    pub page_current: i64,
    pub page_total: i64,
}

impl Pagination {
    pub fn default() -> Self {
        Pagination {
            has_next: false,
            has_prev: false,
            items_current: 0,
            items_total: 0,
            limit: 0,
            page_current: 0,
            page_total: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Count {
    pub count: i64,
}
