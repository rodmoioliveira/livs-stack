use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use std::collections::HashSet;

fn derive_query(v: Vec<String>) -> String {
    let mut q = v.into_iter().filter(|q| *q != "").collect::<Vec<String>>();
    q.sort();
    let q = q.join("&");
    let question_mark = if q.len() == 0 { "" } else { "?" };

    format!("{}{}", question_mark, q)
}

fn set_to_vec(set: &HashSet<i64>) -> Vec<i64> {
    set.clone().into_iter().collect()
}

fn ids_comma_joiner(set: &HashSet<i64>) -> String {
    let mut ids: Vec<i64> = set_to_vec(&set);
    ids.sort();
    ids.iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn ids_set(s: Option<String>) -> HashSet<i64> {
    s.unwrap_or("0".to_string())
        .split(",")
        .map(|s| s.parse::<i64>().unwrap())
        .filter(|id| id != &0_i64)
        .collect::<HashSet<i64>>()
}

// TODO: MUST REFACTOR THIS WHOLE FILE!
pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::Endpoints>,
    web::Query(filter_qs): web::Query<models::Filters>,
) -> Result<HttpResponse, errors::MyError> {
    let genres = utils::fetch(endpoints.backend_url("genres?order_by=genre"), &client)?;
    let languages = utils::fetch(
        endpoints.backend_url("languages?order_by=language"),
        &client,
    )?;
    let formats = utils::fetch(endpoints.backend_url("formats?order_by=format"), &client)?;

    let set_genres = ids_set(filter_qs.clone().genres);
    let set_languages = ids_set(filter_qs.clone().languages);
    let set_formats = ids_set(filter_qs.clone().formats);

    let languages_qs: String = ids_comma_joiner(&set_languages);
    let genres_qs: String = ids_comma_joiner(&set_genres);
    let formats_qs: String = ids_comma_joiner(&set_formats);

    let all_languages: Vec<models::Language> =
        serde_json::from_value(languages["data"].clone()).unwrap();
    let all_genres: Vec<models::Genre> = serde_json::from_value(genres["data"].clone()).unwrap();
    let all_formats: Vec<models::Format> = serde_json::from_value(formats["data"].clone()).unwrap();

    let qs_genres = match set_genres.len() {
        0 => "".to_string(),
        _ => format!("genres={}", genres_qs),
    };

    let qs_languages = match set_languages.len() {
        0 => "".to_string(),
        _ => format!("languages={}", languages_qs),
    };

    let qs_formats = match set_formats.len() {
        0 => "".to_string(),
        _ => format!("formats={}", formats_qs),
    };

    let filter_genres = all_genres
        .iter()
        .map(|genre| {
            let id = genre.id.unwrap();
            let selected = set_genres.contains(&genre.id.unwrap());
            let mut set = set_genres.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = ids_comma_joiner(&set);
            let qs_genres = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("genres={}", qs_values),
            };

            let queries = derive_query(vec![qs_genres, qs_languages.clone(), qs_formats.clone()]);
            let link = format!("/titles{}", queries);

            models::Filter {
                id,
                name: "genre".to_string(),
                selected,
                value: genre.genre.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();

    let filter_languages = all_languages
        .iter()
        .map(|language| {
            let id = language.id.unwrap();
            let selected = set_languages.contains(&language.id.unwrap());
            let mut set = set_languages.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = ids_comma_joiner(&set);
            let qs_languages = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("languages={}", qs_values),
            };

            let queries = derive_query(vec![qs_genres.clone(), qs_languages, qs_formats.clone()]);
            let link = format!("/titles{}", queries);

            models::Filter {
                id,
                name: "language".to_string(),
                selected,
                value: language.language.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();

    let filter_formats = all_formats
        .iter()
        .map(|format| {
            let id = format.id.unwrap();
            let selected = set_formats.contains(&format.id.unwrap());
            let mut set = set_formats.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = ids_comma_joiner(&set);
            let qs_formats = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("formats={}", qs_values),
            };

            let queries = derive_query(vec![qs_genres.clone(), qs_languages.clone(), qs_formats]);
            let link = format!("/titles{}", queries);

            models::Filter {
                id,
                name: "format".to_string(),
                selected,
                value: format.format.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();

    let queries = derive_query(vec![qs_genres, qs_languages, qs_formats]);
    let link = format!("titles{}", queries);
    let titles = utils::fetch(endpoints.backend_url(&link), &client)?;

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "genres": serde_json::json!(filter_genres),
        "languages": serde_json::json!(filter_languages),
        "formats": serde_json::json!(filter_formats),
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
