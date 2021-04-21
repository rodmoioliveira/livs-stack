use crate::{errors, models};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub async fn index(
    hb: web::Data<Handlebars<'_>>,
    endpoints: web::Data<models::types::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let data = serde_json::json!({
        "assets": endpoints.assets,
    });

    let body = hb.render("pages/main", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
