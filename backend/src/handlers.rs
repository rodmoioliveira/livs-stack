use crate::models;
use actix_web::{get, web, HttpResponse, Responder, Result};

#[get("/")]
async fn index() -> Result<String> {
    Ok(format!("index"))
}

#[get("/books")]
async fn books() -> impl Responder {
    HttpResponse::Ok().json(models::Books {
        data: vec![
            models::Book {
                id: 12,
                isbn: "isbn".to_string(),
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                id: 12,
                isbn: "isbn".to_string(),
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                id: 12,
                isbn: "isbn".to_string(),
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                id: 12,
                isbn: "isbn".to_string(),
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
        ],
    })
}

#[get("/book/{isbn}")]
async fn book(web::Path(isbn): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(models::Book {
        id: 12,
        isbn,
        author: "author".to_string(),
        title: "title".to_string(),
        editor: "editor".to_string(),
        description: "description".to_string(),
    })
}
