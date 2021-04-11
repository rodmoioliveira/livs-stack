use crate::errors;
use actix_web::{get, Result};

#[get("/")]
pub async fn index() -> Result<String, errors::MyError> {
    Ok(format!("index"))
}
