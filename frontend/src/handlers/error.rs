use actix_http::{body::Body, Response};
use actix_web::dev::ServiceResponse;
use actix_web::http::StatusCode;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{web, Result};
use handlebars::Handlebars;

pub fn _404() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

fn get_error_response<B>(
    res: &ServiceResponse<B>,
    error: &str,
) -> Response<Body> {
    let request = res.request();
    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };
    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());

    match hb {
        Some(hb) => {
            let data = serde_json::json!({
                "error": error,
                "status_code": res.status().as_str()
            });
            let body = hb.render("pages/404", &data);

            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}
