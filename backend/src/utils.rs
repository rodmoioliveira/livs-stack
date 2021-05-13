use crate::{errors, models, querystrings};
use actix_web::Result;

pub fn handle_pagination(
    count: i64,
    offset: i64,
    limit: i64,
) -> Result<(), errors::MyError> {
    if offset % limit != 0 {
        return Err(errors::MyError::BadOffset);
    };

    let has_items = count > 0;
    let page_total = (count as f64 / limit as f64).ceil() as i64;
    let page_current = if !has_items { 0 } else { (offset / limit) + 1 };

    if page_current > page_total {
        return Err(errors::MyError::BadPagination);
    };

    Ok(())
}

pub fn get_pagination(
    order_by_qs: &querystrings::Order,
    count: i64,
    items_current: i64,
) -> Result<models::db::Pagination, errors::MyError> {
    let offset = order_by_qs.offset.unwrap_or(0);
    let limit = order_by_qs.limit.unwrap_or(count);

    if limit == 0 {
        return Ok(models::db::Pagination {
            has_next: false,
            has_prev: false,
            items_current: 0,
            items_total: 0,
            limit: 0,
            page_current: 0,
            page_total: 0,
        });
    };

    let valid = offset % limit == 0;
    if !valid {
        return Err(errors::MyError::BadPagination);
    };

    let has_items = count > 0;
    let page_total = (count as f64 / limit as f64).ceil() as i64;
    let page_current = if !has_items { 0 } else { (offset / limit) + 1 };

    let has_next = page_current < page_total;
    let has_prev = page_current > 1;

    Ok(models::db::Pagination {
        has_next,
        has_prev,
        items_current,
        items_total: count,
        limit,
        page_current,
        page_total,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pagination() -> Result<(), errors::MyError> {
        let _anwsers = vec![
            models::db::Pagination {
                has_next: true,
                has_prev: false,
                items_current: 5,
                items_total: 14,
                limit: 5,
                page_current: 1,
                page_total: 3,
            },
            models::db::Pagination {
                has_next: true,
                has_prev: true,
                items_current: 5,
                items_total: 14,
                limit: 5,
                page_current: 2,
                page_total: 3,
            },
            models::db::Pagination {
                has_next: false,
                has_prev: true,
                items_current: 4,
                items_total: 14,
                limit: 5,
                page_current: 3,
                page_total: 3,
            },
        ];

        let _qs = vec![
            (
                querystrings::Order {
                    offset: Some(0),
                    limit: Some(5),
                    order_by: None,
                },
                14,
                5,
            ),
            (
                querystrings::Order {
                    offset: Some(5),
                    limit: Some(5),
                    order_by: None,
                },
                14,
                5,
            ),
            (
                querystrings::Order {
                    offset: Some(10),
                    limit: Some(5),
                    order_by: None,
                },
                14,
                4,
            ),
        ];

        let _results: Vec<models::db::Pagination> = _qs
            .into_iter()
            .map(|args| get_pagination(&args.0, args.1, args.2).unwrap())
            .collect();

        _results.iter().zip(_anwsers.iter()).for_each(|a| {
            assert_eq!(a.0, a.1);
        });

        Ok(())
    }
}
