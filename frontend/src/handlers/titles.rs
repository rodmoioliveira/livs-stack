use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// TODO: MUST REFACTOR THIS WHOLE FILE!

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filters {
    pub formats: Option<String>,
    pub genres: Option<String>,
    pub languages: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    pub id: i64,
    pub name: String,
    pub selected: bool,
    pub value: String,
    pub link: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Genre {
    pub id: Option<i64>,
    pub genre: String,
}

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
    web::Query(filter_qs): web::Query<Filters>,
) -> Result<HttpResponse, errors::MyError> {
    let genres = utils::fetch(endpoints.backend_url("genres?order_by=genre"), &client)?;
    let languages = utils::fetch(
        endpoints.backend_url("languages?order_by=language"),
        &client,
    )?;
    let formats = utils::fetch(endpoints.backend_url("formats?order_by=format"), &client)?;

    let selected_genres_set = filter_qs
        .clone()
        .genres
        .unwrap_or("0".to_string())
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .filter(|id| id != &0_i64)
        .collect::<HashSet<i64>>();
    let genres: Vec<Genre> = serde_json::from_value(genres["data"].clone()).unwrap();
    let filter_genres = genres
        .iter()
        .map(|genre| {
            let id = genre.id.unwrap();
            let selected = selected_genres_set.contains(&genre.id.unwrap());
            let mut set = selected_genres_set.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let mut ids: Vec<i64> = set.clone().into_iter().collect();
            ids.sort();
            let qs_values: String = ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<String>>()
                .join(",");

            let qs = match set.len() {
                0 => "".to_string(),
                _ => format!("?genres={}", qs_values),
            };

            let link = format!("/titles{}", qs);

            Filter {
                id,
                name: "genre".to_string(),
                selected,
                value: genre.genre.clone(),
                link,
            }
        })
        .collect::<Vec<Filter>>();

    let qs_values: String = selected_genres_set
        .clone()
        .into_iter()
        .collect::<Vec<i64>>()
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let qs = match selected_genres_set.len() {
        0 => "".to_string(),
        _ => format!("?genres={}", qs_values),
    };

    let link = format!("titles{}", qs);

    let titles = utils::fetch(endpoints.backend_url(&link), &client)?;

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "genres": serde_json::json!(filter_genres),
        "languages": languages["data"],
        "formats": formats["data"],
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
