use crate::{errors, models, querystrings};
use actix_web::Result;

pub fn get_pagination(
    order_by_qs: querystrings::Order,
    count: i64,
    items_current: i64,
) -> Result<models::db::Pagination, errors::MyError> {
    let offset = order_by_qs.offset.unwrap_or(0);
    let limit = order_by_qs.limit.unwrap_or(count);

    let valid = offset % limit == 0;
    if !valid {
        return Err(errors::MyError::BadPagination);
    };

    let items_total = count;
    let page_total = (items_total as f64 / limit as f64).ceil() as i64;
    let page_current = (offset / limit) + 1;
    let has_next = page_current < page_total;
    let has_prev = page_current > 1;

    Ok(models::db::Pagination {
        page_current,
        items_current,
        page_total,
        items_total,
        has_prev,
        has_next,
        limit,
    })
}
