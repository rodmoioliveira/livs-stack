use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Deserialize, Serialize)]
pub struct JsonError<T> {
    pub error: T,
}

impl<T> JsonError<T> {
    pub fn new(err: T) -> Self {
        JsonError { error: err }
    }
}

#[derive(Display, From, Debug)]
pub enum MyError {
    NotFound,
    ReqwestError(ReqwestError),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().json(JsonError::new("Not Found")),
            MyError::ReqwestError(ref err) => {
                HttpResponse::InternalServerError().json(JsonError::new(err.to_string()))
            }
            _ => HttpResponse::InternalServerError().json(JsonError::new("ERROR")),
        }
    }
}
