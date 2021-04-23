use crate::errors;
use actix_web::web;
use reqwest::blocking::Client;

pub fn fetch(
    url: String,
    client: &web::Data<Client>,
) -> Result<serde_json::Value, errors::MyError> {
    Ok(client
        .get(url)
        .send()
        .map_err(errors::MyError::ReqwestError)?
        .json()
        .map_err(errors::MyError::ReqwestError)?)
}
