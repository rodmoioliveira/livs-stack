use crate::{errors, models, querystrings};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(
    client: &Client,
    title_qs: querystrings::titles::Order,
) -> Result<Vec<models::titles::Title>, errors::MyError> {
    let _stmt = include_str!("../sql/titles/all.sql");
    let _stmt = _stmt.replace("$title_qs", &title_qs.to_sql());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let result: Vec<models::titles::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    Ok(result)
}

pub async fn one(client: &Client, id: i64) -> Result<models::titles::Title, errors::MyError> {
    let _stmt = include_str!("../sql/titles/one.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::titles::Title> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn add(
    client: &Client,
    title: models::titles::Title,
) -> Result<models::titles::Title, errors::MyError> {
    let _stmt = include_str!("../sql/titles/add.sql");
    let _stmt = _stmt.replace("$table_fields", &models::titles::Title::sql_table_fields());
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
        .map(|row| models::titles::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::titles::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update(
    client: &Client,
    id: i64,
    title: models::titles::Title,
) -> Result<models::titles::Title, errors::MyError> {
    let _stmt = include_str!("../sql/titles/update.sql");
    let _stmt = _stmt.replace("$table_fields", &models::titles::Title::sql_table_fields());
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
        .map(|row| models::titles::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::titles::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(client: &Client, id: i64) -> Result<models::titles::Title, errors::MyError> {
    let _stmt = include_str!("../sql/titles/delete.sql");
    let _stmt = _stmt.replace("$table_fields", &models::titles::Title::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| models::titles::Title::from_row_ref(row).unwrap())
        .collect::<Vec<models::titles::Title>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
