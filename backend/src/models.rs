use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub isbn: String,
    pub author: String,
    pub title: String,
    pub editor: String,
    pub description: String,
}

#[derive(Deserialize, Serialize)]
pub struct Books {
    pub data: Vec<Book>,
}
