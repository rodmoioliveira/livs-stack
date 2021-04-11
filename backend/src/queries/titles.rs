use crate::{errors, models};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(client: &Client) -> Result<Vec<models::Title>, errors::MyError> {
    let _stmt = include_str!("../sql/get_titles.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(result)
}

pub async fn one(client: &Client, id: i64) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("../sql/get_title.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn add(client: &Client, title: models::Title) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("../sql/insert_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[
                &title.isbn,
                &title.author,
                &title.edition,
                &title.format,
                &title.language,
                &title.genre,
                &title.pages,
                &title.publisher,
                &title.summary,
                &title.title,
                &title.year,
            ],
        )
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update(
    client: &Client,
    id: i64,
    title: models::Title,
) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("../sql/update_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[
                &title.isbn,
                &title.author,
                &title.edition,
                &title.format,
                &title.language,
                &title.genre,
                &title.pages,
                &title.publisher,
                &title.summary,
                &title.title,
                &title.year,
                &id,
            ],
        )
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(client: &Client, id: i64) -> Result<models::Title, errors::MyError> {
    let _stmt = include_str!("../sql/delete_title.sql");
    let _stmt = _stmt.replace("$table_fields", &models::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| models::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
