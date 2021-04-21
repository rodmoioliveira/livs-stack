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
        .get(format!("{}/titles", endpoints.backend))
        .send()
        .map_err(errors::MyError::ReqwestError)?
        .json()
        .map_err(errors::MyError::ReqwestError)?;

    let data = serde_json::json!({
        // TODO: get within docker container for prod
        // https://docs.docker.com/compose/compose-file/compose-file-v3/#ipv4_address-ipv6_address
        "assets": endpoints.assets,
        "titles": res["data"],
    });

    let body = hb.render("titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
