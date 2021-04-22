use crate::{errors, models};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let res: serde_json::Value = client
        .get(format!("{}/formats", endpoints.backend))
        .send()
        .map_err(errors::MyError::ReqwestError)?
        .json()
        .map_err(errors::MyError::ReqwestError)?;

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "formats": res["data"],
    });

    let body = hb.render("pages/formats", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}