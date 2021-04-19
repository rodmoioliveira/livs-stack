use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = serde_json::json!({
        "name": "Handlebars"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}
