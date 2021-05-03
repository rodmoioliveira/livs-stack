use crate::{errors, models, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;
use std::collections::HashSet;

// TODO: MUST REFACTOR THIS WHOLE FILE!
pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::types::Endpoints>,
    web::Query(filter_qs): web::Query<models::types::Filters>,
) -> Result<HttpResponse, errors::MyError> {
    let sets = utils::fetch(endpoints.backend_url("sets"), &client)?;
    let genres = utils::fetch(endpoints.backend_url("genres?order_by=genre"), &client)?;
    let languages = utils::fetch(
        endpoints.backend_url("languages?order_by=language"),
        &client,
    )?;
    let formats = utils::fetch(endpoints.backend_url("formats?order_by=format"), &client)?;

    let qs_set_genres = utils::ids_set(filter_qs.clone().genres);
    let qs_set_languages = utils::ids_set(filter_qs.clone().languages);
    let qs_set_formats = utils::ids_set(filter_qs.clone().formats);

    let languages_qs: String = utils::ids_comma_joiner(&qs_set_languages);
    let genres_qs: String = utils::ids_comma_joiner(&qs_set_genres);
    let formats_qs: String = utils::ids_comma_joiner(&qs_set_formats);

    let all_languages: Vec<models::types::Language> =
        serde_json::from_value(languages["data"].clone()).unwrap();
    let all_genres: Vec<models::types::Genre> =
        serde_json::from_value(genres["data"].clone()).unwrap();
    let all_formats: Vec<models::types::Format> =
        serde_json::from_value(formats["data"].clone()).unwrap();

    let all_languages_set: HashSet<i64> = all_languages
        .clone()
        .iter()
        .map(|i| i.id.unwrap())
        .collect();
    let all_genres_set: HashSet<i64> = all_genres.clone().iter().map(|i| i.id.unwrap()).collect();
    let all_formats_set: HashSet<i64> = all_formats.clone().iter().map(|i| i.id.unwrap()).collect();

    let all_sets: models::types::Sets = serde_json::from_value(sets["data"].clone()).unwrap();

    let qs_genres = match qs_set_genres.len() {
        0 => "".to_string(),
        _ => format!("genres={}", genres_qs),
    };

    let qs_languages = match qs_set_languages.len() {
        0 => "".to_string(),
        _ => format!("languages={}", languages_qs),
    };

    let qs_formats = match qs_set_formats.len() {
        0 => "".to_string(),
        _ => format!("formats={}", formats_qs),
    };

    let mut navlinks_genre = all_genres
        .iter()
        .map(|genre| {
            let id = genre.id.unwrap();
            let selected = qs_set_genres.contains(&genre.id.unwrap());
            let mut set = qs_set_genres.clone();

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

            models::types::NavLink {
                id,
                name: "genre".to_string(),
                selected,
                value: genre.genre.clone(),
                link,
            }
        })
        .collect::<Vec<models::types::NavLink>>();

    let mut navlinks_language = all_languages
        .iter()
        .map(|language| {
            let id = language.id.unwrap();
            let selected = qs_set_languages.contains(&language.id.unwrap());
            let mut set = qs_set_languages.clone();

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

            models::types::NavLink {
                id,
                name: "language".to_string(),
                selected,
                value: language.language.clone(),
                link,
            }
        })
        .collect::<Vec<models::types::NavLink>>();

    let mut navlinks_format = all_formats
        .iter()
        .map(|format| {
            let id = format.id.unwrap();
            let selected = qs_set_formats.contains(&format.id.unwrap());
            let mut set = qs_set_formats.clone();

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

            models::types::NavLink {
                id,
                name: "format".to_string(),
                selected,
                value: format.format.clone(),
                link,
            }
        })
        .collect::<Vec<models::types::NavLink>>();

    let queries = utils::derive_query(vec![qs_genres, qs_languages, qs_formats]);
    let link = format!("titles{}", queries);
    let titles = utils::fetch(endpoints.backend_url(&link), &client)?;

    let all_titles: Vec<models::types::Title> =
        serde_json::from_value(titles["data"].clone()).unwrap();
    let res_format_set: HashSet<i64> = all_titles.clone().iter().map(|i| i.format as i64).collect();
    let res_genre_set: HashSet<i64> = all_titles.clone().iter().map(|i| i.genre as i64).collect();
    let res_language_set: HashSet<i64> = all_titles
        .clone()
        .iter()
        .map(|i| i.language as i64)
        .collect();

    let g_is_active = qs_set_genres.len() > 0;
    let l_is_active = qs_set_languages.len() > 0;
    let f_is_active = qs_set_formats.len() > 0;

    let mut g_vsets = models::types::SetVec {
        format: vec![],
        genre: vec![],
        language: vec![],
    };
    let mut l_vsets = models::types::SetVec {
        format: vec![],
        genre: vec![],
        language: vec![],
    };
    let mut f_vsets = models::types::SetVec {
        format: vec![],
        genre: vec![],
        language: vec![],
    };

    if l_is_active {
        let ids = qs_set_languages.clone().into_iter();

        ids.for_each(|id| {
            let g_set = &all_sets.language.get(&id).unwrap().genre;
            let f_set = &all_sets.language.get(&id).unwrap().format;

            l_vsets.genre.push(g_set.clone());
            l_vsets.format.push(f_set.clone());
        });
    }

    if g_is_active {
        let ids = qs_set_genres.clone().into_iter();

        ids.for_each(|id| {
            let l_set = &all_sets.genre.get(&id).unwrap().language;
            let f_set = &all_sets.genre.get(&id).unwrap().format;

            g_vsets.language.push(l_set.clone());
            g_vsets.format.push(f_set.clone());
        });
    }

    if f_is_active {
        let ids = qs_set_formats.clone().into_iter();

        ids.for_each(|id| {
            let g_set = &all_sets.format.get(&id).unwrap().genre;
            let l_set = &all_sets.format.get(&id).unwrap().language;

            f_vsets.language.push(l_set.clone());
            f_vsets.genre.push(g_set.clone());
        });
    }

    g_vsets.union();
    l_vsets.union();
    f_vsets.union();

    let l_itersec = vec![
        g_vsets.language.first().unwrap(),
        f_vsets.language.first().unwrap(),
    ]
    .iter()
    .filter(|hs| hs.len() > 0)
    .fold(all_languages_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });

    let f_itersec = vec![
        g_vsets.format.first().unwrap(),
        l_vsets.format.first().unwrap(),
    ]
    .iter()
    .filter(|hs| hs.len() > 0)
    .fold(all_formats_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });

    let g_itersec = vec![
        f_vsets.genre.first().unwrap(),
        l_vsets.genre.first().unwrap(),
    ]
    .iter()
    .filter(|hs| hs.len() > 0)
    .fold(all_genres_set, |acc, hs| {
        acc.intersection(hs).cloned().collect()
    });

    navlinks_genre = navlinks_genre
        .into_iter()
        .filter(|f| {
            if f_is_active || l_is_active {
                return g_itersec.contains(&f.id) || qs_set_genres.contains(&f.id);
            };

            true
        })
        .filter(|f| {
            if f_is_active && l_is_active {
                return res_genre_set.contains(&f.id) || qs_set_genres.contains(&f.id);
            };

            true
        })
        .collect::<Vec<models::types::NavLink>>();

    navlinks_language = navlinks_language
        .into_iter()
        .filter(|f| {
            if f_is_active || g_is_active {
                return l_itersec.contains(&f.id) || qs_set_languages.contains(&f.id);
            };

            true
        })
        .filter(|f| {
            if f_is_active && g_is_active {
                return res_language_set.contains(&f.id) || qs_set_languages.contains(&f.id);
            };

            true
        })
        .collect::<Vec<models::types::NavLink>>();

    navlinks_format = navlinks_format
        .into_iter()
        .filter(|f| {
            if l_is_active || g_is_active {
                return f_itersec.contains(&f.id) || qs_set_formats.contains(&f.id);
            };

            true
        })
        .filter(|f| {
            if l_is_active && g_is_active {
                return res_format_set.contains(&f.id) || qs_set_formats.contains(&f.id);
            };

            true
        })
        .collect::<Vec<models::types::NavLink>>();

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "genres": serde_json::json!(navlinks_genre),
        "languages": serde_json::json!(navlinks_language),
        "formats": serde_json::json!(navlinks_format),
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
