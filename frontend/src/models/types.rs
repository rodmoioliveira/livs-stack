use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::env;

#[derive(Debug, Clone)]
pub struct Endpoints {
    pub assets: String,
    pub backend: String,
}

impl Endpoints {
    pub fn new() -> Self {
        let assets = format!("http://{}", env::var("ENDPOINT_ASSETS").unwrap());
        let backend = format!("http://{}", env::var("ENDPOINT_BACKEND").unwrap());
        Self { assets, backend }
    }

    pub fn backend_url(
        &self,
        route: &str,
    ) -> String {
        format!("{}/{}", self.backend, route)
    }

    pub fn assets_url(
        &self,
        route: &str,
    ) -> String {
        format!("{}/{}", self.assets, route)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    pub id: i64,
    pub name: String,
    pub selected: bool,
    pub value: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: Option<i64>,
    pub genre: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: Option<i64>,
    pub language: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Format {
    pub id: Option<i64>,
    pub format: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Set {
    pub format: HashSet<i64>,
    pub genre: HashSet<i64>,
    pub language: HashSet<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sets {
    pub language: HashMap<i64, Set>,
    pub genre: HashMap<i64, Set>,
    pub format: HashMap<i64, Set>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetVec {
    pub format: Vec<HashSet<i64>>,
    pub genre: Vec<HashSet<i64>>,
    pub language: Vec<HashSet<i64>>,
}

impl SetVec {
    pub fn union(&mut self) {
        let format = self
            .format
            .iter()
            .fold(HashSet::new(), |acc, hs| acc.union(hs).cloned().collect());
        let genre = self
            .genre
            .iter()
            .fold(HashSet::new(), |acc, hs| acc.union(hs).cloned().collect());
        let language = self
            .language
            .iter()
            .fold(HashSet::new(), |acc, hs| acc.union(hs).cloned().collect());

        self.format = vec![format];
        self.genre = vec![genre];
        self.language = vec![language];
    }
}
