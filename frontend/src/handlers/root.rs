use crate::{errors, models};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    endpoints: web::Data<models::types::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let data = serde_json::json!({
        // TODO: get within docker container for prod
        // https://docs.docker.com/compose/compose-file/compose-file-v3/#ipv4_address-ipv6_address
        "assets": endpoints.assets,
    });

    let body = hb.render("index", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
