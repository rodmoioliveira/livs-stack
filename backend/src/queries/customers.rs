use crate::{errors, models, querystrings, utils};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn all(
    client: &Client,
    order_by_qs: &querystrings::Order,
) -> Result<(Vec<models::db::Customer>, i64), errors::MyError> {
    let _stmt = include_str!("../sql/customers/all.sql");
    let _stmt = _stmt.replace("$order_by", &order_by_qs.to_sql());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[])
        .await
        .map_err(errors::MyError::PGError)?;
    let counts: Vec<models::db::Count> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;
    let count = counts
        .first()
        .unwrap_or(&models::db::Count { count: 0 })
        .count;

    let offset = order_by_qs.offset.unwrap_or(0);
    let limit = order_by_qs.limit.unwrap_or(count);
    if count == 0 || limit == 0 {
        return Ok((vec![], count));
    };

    utils::handle_pagination(count, offset, limit)?;

    let result: Vec<models::db::Customer> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;
    Ok((result, count))
}

pub async fn one(
    client: &Client,
    id: i64,
) -> Result<models::db::Customer, errors::MyError> {
    let _stmt = include_str!("../sql/customers/one.sql");
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;
    let rows = client
        .query(&stmt, &[&id])
        .await
        .map_err(errors::MyError::PGError)?;
    let mut result: Vec<models::db::Customer> =
        serde_postgres::from_rows(&rows).map_err(errors::MyError::PGSerdeError)?;

    result.pop().ok_or(errors::MyError::NotFound)
}

pub async fn add(
    client: &Client,
    customer: models::db::Customer,
) -> Result<models::db::Customer, errors::MyError> {
    let _stmt = include_str!("../sql/customers/add.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Customer::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[&customer.first_name, &customer.last_name, &customer.email],
        )
        .await?
        .iter()
        .map(|row| models::db::Customer::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Customer>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn update(
    client: &Client,
    id: i64,
    customer: models::db::Customer,
) -> Result<models::db::Customer, errors::MyError> {
    let _stmt = include_str!("../sql/customers/update.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Customer::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(
            &stmt,
            &[
                &customer.first_name,
                &customer.last_name,
                &customer.email,
                &id,
            ],
        )
        .await?
        .iter()
        .map(|row| models::db::Customer::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Customer>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}

pub async fn delete(
    client: &Client,
    id: i64,
) -> Result<models::db::Customer, errors::MyError> {
    let _stmt = include_str!("../sql/customers/delete.sql");
    let _stmt = _stmt.replace("$table_fields", &models::db::Customer::sql_table_fields());
    let stmt = client
        .prepare(&_stmt)
        .await
        .map_err(errors::MyError::PGError)?;

    client
        .query(&stmt, &[&id])
        .await?
        .iter()
        .map(|row| models::db::Customer::from_row_ref(row).unwrap())
        .collect::<Vec<models::db::Customer>>()
        .pop()
        .ok_or(errors::MyError::NotFound) // more applicable for SELECTs
}
