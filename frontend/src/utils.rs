use crate::errors;
use actix_web::web;
use reqwest::blocking::Client;
use std::collections::HashSet;

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

pub fn ids_set(s: Option<String>) -> HashSet<i64> {
    s.unwrap_or("0".to_string())
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .filter(|id| id != &0_i64)
        .collect::<HashSet<i64>>()
}
