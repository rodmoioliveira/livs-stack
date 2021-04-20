use crate::errors;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data<T> {
    pub data: T,
}

impl<T> Data<T> {
    pub fn new(data: T) -> Self {
        Data { data }
    }
}

#[derive(Debug, Deserialize, Serialize)]
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

pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
) -> Result<HttpResponse, errors::MyError> {
    let res: serde_json::Value = client
        .get("http://localhost:8081/titles")
        .send()
        .map_err(errors::MyError::ReqwestError)?
        .json()
        .map_err(errors::MyError::ReqwestError)?;

    let data = serde_json::json!({
        // TODO: get within docker container for prod
        "assets": "http://localhost:8082/static",
        "titles": res["data"],
    });

    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
