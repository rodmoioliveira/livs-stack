use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Deserialize, PostgresMapper, Serialize, Clone)]
#[pg_mapper(table = "sets_formats")]
pub struct SetFormat {
    pub id: i16,
    pub genre_set: String,
    pub language_set: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize, Clone)]
#[pg_mapper(table = "sets_languages")]
pub struct SetLanguage {
    pub id: i64,
    pub genre_set: String,
    pub format_set: String,
}

#[derive(Debug, Deserialize, PostgresMapper, Serialize, Clone)]
#[pg_mapper(table = "sets_genres")]
pub struct SetGenre {
    pub id: i64,
    pub format_set: String,
    pub language_set: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sets {
    pub language: HashMap<i64, SetLanguage>,
    pub genre: HashMap<i64, SetGenre>,
    pub format: HashMap<i16, SetFormat>,
}
