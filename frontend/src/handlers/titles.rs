use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let genres = utils::fetch(endpoints.backend_url("genres?order_by=genre"), &client)?;
    let languages = utils::fetch(
        endpoints.backend_url("languages?order_by=language"),
        &client,
    )?;
    let formats = utils::fetch(endpoints.backend_url("formats?order_by=format"), &client)?;
    let titles = utils::fetch(endpoints.backend_url("titles"), &client)?;

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "genres": genres["data"],
        "languages": languages["data"],
        "formats": formats["data"],
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
