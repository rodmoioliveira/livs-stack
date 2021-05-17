use serde::{Deserialize, Serialize};
use std::env;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Page {
    pub active: bool,
    pub link: String,
    pub number: i64,
    pub selected: bool,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageControl {
    pub active: bool,
    pub link: String,
    pub value: String,
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
        format!("{}{}", self.backend, route)
    }

    pub fn assets_url(
        &self,
        route: &str,
    ) -> String {
        format!("{}{}", self.assets, route)
    }
}
