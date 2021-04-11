use crate::errors;
use actix_web::Result;

pub async fn index() -> Result<String, errors::MyError> {
    Ok(format!("index"))
}
