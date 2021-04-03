use crate::{errors, models};
use actix_web::{get, web, Error, HttpResponse, Responder, Result};
use deadpool_postgres::{Client, Pool};

#[get("/")]
async fn index() -> Result<String> {
    Ok(format!("index"))
}

#[get("/books")]
async fn books() -> impl Responder {
    HttpResponse::Ok().json(models::Books {
        data: vec![
            models::Book {
                isbn: 12,
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                isbn: 12,
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                isbn: 12,
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
            models::Book {
                isbn: 12,
                author: "author".to_string(),
                title: "title".to_string(),
                editor: "editor".to_string(),
                description: "description".to_string(),
            },
        ],
    })
}

#[get("/book/{isbn}")]
async fn book(
    web::Path(isbn): web::Path<String>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(errors::MyError::PoolError)?;
    let query: String = format!("SELECT * FROM books WHERE isbn = '{}'", isbn);
    let stmt = client.prepare(&query).await.unwrap();
    let rows = client.query(&stmt, &[]).await.unwrap();
    let book: Vec<models::Book> = serde_postgres::from_rows(&rows).unwrap();

    Ok(HttpResponse::Ok().json(book))
}
