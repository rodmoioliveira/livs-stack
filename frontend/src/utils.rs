use crate::{errors, models};
use actix_web::web;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use reqwest::blocking::Client;
use std::collections::HashSet;

pub fn count_active(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let filters = h.param(0).unwrap().value();
    let f: Vec<models::Filter> = serde_json::from_value(filters.clone()).unwrap();
    let count = f.into_iter().filter(|f| f.selected).count();

    out.write(&count.to_string())?;
    Ok(())
}

pub fn fetch(
    url: String,
    client: &web::Data<Client>,
) -> Result<serde_json::Value, errors::MyError> {
    Ok(client
        .get(url)
        .send()
        .map_err(errors::MyError::ReqwestError)?
        .json()
        .map_err(errors::MyError::ReqwestError)?)
}

pub fn derive_link(
    path: &str,
    queries_ids: Vec<String>,
) -> String {
    let mut q = queries_ids
        .into_iter()
        .filter(|q| *q != "")
        .collect::<Vec<String>>();
    q.sort();
    let q = q.join("&");
    let question_mark = if q.len() == 0 { "" } else { "?" };

    let qs = format!("{}{}", question_mark, q);
    format!("{}{}", path, qs)
}

pub fn derive_query_params(
    query_attr: &str,
    set: &HashSet<i64>,
) -> String {
    let mut ids: Vec<i64> = set.clone().into_iter().collect();
    ids.sort();
    let ids_string = ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    match ids_string.len() {
        0 => "".to_string(),
        _ => format!("{}={}", query_attr, ids_string),
    }
}

pub fn derive_limit_offset(
    predicate: bool,
    limit: i64,
    offset: i64,
) -> (String, String) {
    let mut qp_limit = "".to_string();
    let mut qp_offset = "".to_string();

    if predicate {
        qp_limit = format!("limit={}", limit);
        qp_offset = format!("offset={}", offset);
    }

    (qp_limit, qp_offset)
}

pub fn ids_set(s: Option<String>) -> HashSet<i64> {
    s.unwrap_or("0".to_string())
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .filter(|id| id != &0_i64)
        .collect::<HashSet<i64>>()
}

pub fn get_sym_diff(
    id: i64,
    set: &HashSet<i64>,
) -> HashSet<i64> {
    let mut s = HashSet::new();
    s.insert(id);
    set.symmetric_difference(&s).cloned().collect()
}

pub fn derive_page_controls(
    has_next: bool,
    has_prev: bool,
    limit: i64,
    page_current: i64,
    qps: Vec<String>,
) -> (Vec<models::PageControl>, Vec<models::PageControl>) {
    let prev = models::PageControl {
        active: has_prev,
        link: match has_prev {
            true => {
                let (qp_limit, qp_offset) =
                    derive_limit_offset(page_current != 2, limit, (page_current - 2) * limit);

                let link = derive_link(
                    "/titles",
                    vec![qp_limit, qp_offset]
                        .into_iter()
                        .chain(qps.clone().into_iter())
                        .collect::<Vec<String>>(),
                );

                link
            }
            false => "".to_string(),
        },
        value: "prev".to_string(),
    };

    let next = models::PageControl {
        active: has_next,
        link: match has_next {
            true => {
                let (qp_limit, qp_offset) = derive_limit_offset(true, limit, page_current * limit);

                let link = derive_link(
                    "/titles",
                    vec![qp_limit, qp_offset]
                        .into_iter()
                        .chain(qps.clone().into_iter())
                        .collect::<Vec<String>>(),
                );

                link
            }
            false => "".to_string(),
        },
        value: "next".to_string(),
    };

    let page_control_prev = vec![prev];
    let page_control_next = vec![next];

    (page_control_prev, page_control_next)
}

pub fn derive_pages(
    items_total: i64,
    limit: i64,
    page_current: i64,
    page_total: i64,
    qps: Vec<String>,
) -> Vec<models::Page> {
    let mut pages: Vec<models::Page> = (0..page_total)
        .map(|v| {
            let (qp_limit, qp_offset) = derive_limit_offset(v > 0, limit, v * limit);

            let link = derive_link(
                "/titles",
                vec![qp_limit, qp_offset]
                    .into_iter()
                    .chain(qps.clone().into_iter())
                    .collect::<Vec<String>>(),
            );

            let page_number = v + 1;

            models::Page {
                active: page_current != page_number,
                link,
                index: page_number,
                selected: page_current == page_number,
                value: page_number.to_string(),
            }
        })
        .collect();

    if pages.len() > 3 {
        let mut pages_copy: Vec<models::Page> = pages.clone();
        let page_first = pages_copy.remove(0);
        let page_last = pages_copy.pop().unwrap();

        let inner_pages: Vec<models::Page> = pages_copy
            .iter()
            .cloned()
            .filter(|p| {
                let is_edge = page_current < 5 || page_current > page_total - 4;
                let offset_range = if is_edge { 4 } else { 3 };

                p.index > page_current - offset_range && p.index < page_current + offset_range
            })
            .collect();

        let mut inner_pages_copy = inner_pages.clone();
        let page_second = inner_pages_copy.remove(0);
        let page_penultimate = inner_pages_copy.pop().unwrap();

        let discontinuous_head = page_second.index - page_first.index > 1;
        let discontinuous_tail = page_last.index - page_penultimate.index > 1;

        let first_ellipsis: Vec<models::Page> = match discontinuous_head {
            true => {
                let mut offset = limit * (page_current - 6);
                let is_out_of_bound = offset <= 0;
                offset = match is_out_of_bound {
                    true => 0,
                    false => offset,
                };
                let (qp_limit, qp_offset) = derive_limit_offset(!is_out_of_bound, limit, offset);

                let link = derive_link(
                    "/titles",
                    vec![qp_limit, qp_offset]
                        .into_iter()
                        .chain(qps.clone().into_iter())
                        .collect::<Vec<String>>(),
                );

                vec![models::Page {
                    active: true,
                    link,
                    index: 0,
                    selected: false,
                    value: "<<".to_string(),
                }]
            }
            false => vec![],
        };

        let second_ellipsis: Vec<models::Page> = match discontinuous_tail {
            true => {
                let mut offset = limit * (page_current + 4);
                let is_out_of_bound = offset >= items_total;
                offset = match is_out_of_bound {
                    true => offset - limit,
                    false => offset,
                };
                let (qp_limit, qp_offset) = derive_limit_offset(true, limit, offset);

                let link = derive_link(
                    "/titles",
                    vec![qp_limit, qp_offset]
                        .into_iter()
                        .chain(qps.clone().into_iter())
                        .collect::<Vec<String>>(),
                );

                vec![models::Page {
                    active: true,
                    link,
                    index: 0,
                    selected: false,
                    value: ">>".to_string(),
                }]
            }
            false => vec![],
        };

        pages = vec![
            vec![page_first],
            first_ellipsis,
            inner_pages,
            second_ellipsis,
            vec![page_last],
        ]
        .into_iter()
        .flatten()
        .collect();
    }

    pages
}
