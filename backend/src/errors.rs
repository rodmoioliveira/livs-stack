use actix_web::{HttpResponse, ResponseError};
use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use serde_postgres::DeError as PGSerdeError;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

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
    BadOffset,
    BadPagination,
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PGSerdeError(PGSerdeError),
    PoolError(PoolError),
}

impl std::error::Error for MyError {}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::NotFound => HttpResponse::NotFound().json(JsonError::new("Not Found")),
            MyError::BadOffset => HttpResponse::BadRequest()
                .json(JsonError::new("Bad offset: offset % limit is not 0.")),
            MyError::BadPagination => HttpResponse::BadRequest()
                .json(JsonError::new("Bad pagination: page_current > page_total.")),
            MyError::PGError(ref err) => {
                HttpResponse::InternalServerError().json(JsonError::new(err.to_string()))
            }
            MyError::PGSerdeError(ref err) => {
                HttpResponse::InternalServerError().json(JsonError::new(err.to_string()))
            }
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().json(JsonError::new(err.to_string()))
            }
            _ => HttpResponse::InternalServerError().json(JsonError::new("ERROR")),
        }
    }
}
