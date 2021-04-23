use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let res = utils::fetch(endpoints.backend_url("genres"), &client)?;
    let data = serde_json::json!({
        "assets": endpoints.assets,
        "genres": res["data"],
    });

    let body = hb.render("pages/genres", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
