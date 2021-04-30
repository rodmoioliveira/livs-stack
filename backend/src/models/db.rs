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
#[pg_mapper(table = "sets_formats,sets_languages,sets_genres")]
pub struct Set {
    pub format: String,
    pub genre: String,
    pub language: String,
}

impl Set {
    pub fn to_vec(self) -> SetVec {
        let format: Vec<i64> = self
            .format
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let genre: Vec<i64> = self
            .genre
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let language: Vec<i64> = self
            .language
            .split(",")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        SetVec {
            format,
            genre,
            language,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetVec {
    pub format: Vec<i64>,
    pub genre: Vec<i64>,
    pub language: Vec<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sets {
    pub language: HashMap<String, SetVec>,
    pub genre: HashMap<String, SetVec>,
    pub format: HashMap<String, SetVec>,
}
