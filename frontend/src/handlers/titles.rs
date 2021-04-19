use actix_web::{web, HttpResponse};
use handlebars::Handlebars;

pub async fn user(
    hb: web::Data<Handlebars<'_>>,
    web::Path(info): web::Path<(String, String)>,
) -> HttpResponse {
    let data = serde_json::json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}
