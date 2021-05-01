use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Language {
    pub id: Option<i64>,
    pub language: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Format {
    pub id: Option<i64>,
    pub format: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Title {
    pub id: Option<i64>,
    pub isbn: String,
    pub author: i64,
    pub edition: i16,
    pub format: i16,
    pub language: i64,
    pub genre: i64,
    pub pages: i16,
    pub publisher: i64,
    pub summary: String,
    pub title: String,
    pub year: i16,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Set {
    pub format: HashSet<i64>,
    pub genre: HashSet<i64>,
    pub language: HashSet<i64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sets {
    pub language: HashMap<i64, Set>,
    pub genre: HashMap<i64, Set>,
    pub format: HashMap<i64, Set>,
}

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
    web::Query(filter_qs): web::Query<Filters>,
) -> Result<HttpResponse, errors::MyError> {
    let sets = utils::fetch(endpoints.backend_url("sets"), &client)?;
    let genres = utils::fetch(endpoints.backend_url("genres?order_by=genre"), &client)?;
    let languages = utils::fetch(
        endpoints.backend_url("languages?order_by=language"),
        &client,
    )?;
    let formats = utils::fetch(endpoints.backend_url("formats?order_by=format"), &client)?;

    let set_genres = utils::ids_set(filter_qs.clone().genres);
    let set_languages = utils::ids_set(filter_qs.clone().languages);
    let set_formats = utils::ids_set(filter_qs.clone().formats);

    let languages_qs: String = utils::ids_comma_joiner(&set_languages);
    let genres_qs: String = utils::ids_comma_joiner(&set_genres);
    let formats_qs: String = utils::ids_comma_joiner(&set_formats);

    let all_languages: Vec<Language> = serde_json::from_value(languages["data"].clone()).unwrap();
    let all_genres: Vec<Genre> = serde_json::from_value(genres["data"].clone()).unwrap();
    let all_formats: Vec<Format> = serde_json::from_value(formats["data"].clone()).unwrap();

    let all_languages_set: HashSet<i64> = all_languages
        .clone()
        .iter()
        .map(|i| i.id.unwrap())
        .collect();
    let all_genres_set: HashSet<i64> = all_genres.clone().iter().map(|i| i.id.unwrap()).collect();
    let all_formats_set: HashSet<i64> = all_formats.clone().iter().map(|i| i.id.unwrap()).collect();

    let all_sets: Sets = serde_json::from_value(sets["data"].clone()).unwrap();

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

    let mut filter_genres = all_genres
        .iter()
        .map(|genre| {
            let id = genre.id.unwrap();
            let selected = set_genres.contains(&genre.id.unwrap());
            let mut set = set_genres.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = utils::ids_comma_joiner(&set);
            let qs_genres = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("genres={}", qs_values),
            };

            let queries =
                utils::derive_query(vec![qs_genres, qs_languages.clone(), qs_formats.clone()]);
            let link = format!("/titles{}", queries);

            Filter {
                id,
                name: "genre".to_string(),
                selected,
                value: genre.genre.clone(),
                link,
            }
        })
        .collect::<Vec<Filter>>();

    let mut filter_languages = all_languages
        .iter()
        .map(|language| {
            let id = language.id.unwrap();
            let selected = set_languages.contains(&language.id.unwrap());
            let mut set = set_languages.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = utils::ids_comma_joiner(&set);
            let qs_languages = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("languages={}", qs_values),
            };

            let queries =
                utils::derive_query(vec![qs_genres.clone(), qs_languages, qs_formats.clone()]);
            let link = format!("/titles{}", queries);

            Filter {
                id,
                name: "language".to_string(),
                selected,
                value: language.language.clone(),
                link,
            }
        })
        .collect::<Vec<Filter>>();

    let mut filter_formats = all_formats
        .iter()
        .map(|format| {
            let id = format.id.unwrap();
            let selected = set_formats.contains(&format.id.unwrap());
            let mut set = set_formats.clone();

            match selected {
                true => set.remove(&id),
                false => set.insert(id),
            };

            let qs_values = utils::ids_comma_joiner(&set);
            let qs_formats = match qs_values.len() {
                0 => "".to_string(),
                _ => format!("formats={}", qs_values),
            };

            let queries =
                utils::derive_query(vec![qs_genres.clone(), qs_languages.clone(), qs_formats]);
            let link = format!("/titles{}", queries);

            Filter {
                id,
                name: "format".to_string(),
                selected,
                value: format.format.clone(),
                link,
            }
        })
        .collect::<Vec<Filter>>();

    let queries = utils::derive_query(vec![qs_genres, qs_languages, qs_formats]);
    let link = format!("titles{}", queries);
    let titles = utils::fetch(endpoints.backend_url(&link), &client)?;

    let genres_is_active = set_genres.len() > 0;
    let language_is_active = set_languages.len() > 0;
    let format_is_active = set_formats.len() > 0;

    // println!("=========================");
    // println!(
    //     "formats => active?: {}, values: {:#?}",
    //     format_is_active, set_formats
    // );
    // println!(
    //     "languages => active?: {}, values: {:#?}",
    //     language_is_active, set_languages
    // );
    // println!(
    //     "genres => active?: {}, values: {:#?}",
    //     genres_is_active, set_genres
    // );
    // println!("=========================");

    let mut g_sets: Vec<HashSet<i64>> = vec![];
    let mut l_sets: Vec<HashSet<i64>> = vec![];
    let mut f_sets: Vec<HashSet<i64>> = vec![];

    if language_is_active {
        let ids = set_languages.into_iter();

        ids.for_each(|id| {
            let g_set = &all_sets.language.get(&id).unwrap().genre;
            let f_set = &all_sets.language.get(&id).unwrap().format;

            g_sets.push(g_set.clone());
            f_sets.push(f_set.clone());
        });
    }

    if genres_is_active {
        let ids = set_genres.into_iter();

        ids.for_each(|id| {
            let l_set = &all_sets.genre.get(&id).unwrap().language;
            let f_set = &all_sets.genre.get(&id).unwrap().format;

            l_sets.push(l_set.clone());
            f_sets.push(f_set.clone());
        });
    }

    if format_is_active {
        let ids = set_formats.into_iter();

        ids.for_each(|id| {
            let g_set = &all_sets.format.get(&id).unwrap().genre;
            let l_set = &all_sets.format.get(&id).unwrap().language;

            g_sets.push(g_set.clone());
            l_sets.push(l_set.clone());
        });
    }

    println!("g_sets {:?}", g_sets);
    println!("l_sets {:?}", l_sets);
    println!("f_sets {:?}", f_sets);

    let l_itersec = l_sets.iter().fold(all_languages_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });
    let f_itersec = f_sets.iter().fold(all_formats_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });
    let g_itersec = g_sets.iter().fold(all_genres_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });

    println!("l_itersec {:?}", l_itersec);
    println!("f_itersec {:?}", f_itersec);
    println!("g_itersec {:?}", g_itersec);

    filter_genres = filter_genres
        .into_iter()
        .filter(|f| g_itersec.contains(&f.id))
        .collect::<Vec<Filter>>();
    filter_languages = filter_languages
        .into_iter()
        .filter(|f| l_itersec.contains(&f.id))
        .collect::<Vec<Filter>>();
    filter_formats = filter_formats
        .into_iter()
        .filter(|f| f_itersec.contains(&f.id))
        .collect::<Vec<Filter>>();

    // TODO: fix this case!
    // http://localhost:8083/titles?formats=4&genres=3,32&languages=5
    // travel -> french -> digital access code -> business & money
    // travel -> french -> arts & photography

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
