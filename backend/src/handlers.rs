use actix_web::{get, web, Result};

#[get("/")]
async fn index() -> Result<String> {
    Ok(format!("index"))
}

#[get("/books")]
async fn books() -> Result<String> {
    Ok(format!("All books"))
}

#[get("/book/{isbn}")]
async fn book(web::Path(isbn): web::Path<String>) -> Result<String> {
    Ok(format!("Book isbn: {}", isbn))
}
