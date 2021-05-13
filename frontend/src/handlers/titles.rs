use crate::{errors, models, querystrings, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

// TODO: MUST REFACTOR THIS WHOLE FILE!
pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::Endpoints>,
    web::Query(filter_qs): web::Query<querystrings::Filters>,
) -> Result<HttpResponse, errors::MyError> {
    let set_genres = utils::ids_set(filter_qs.clone().genres);
    let set_languages = utils::ids_set(filter_qs.clone().languages);
    let set_formats = utils::ids_set(filter_qs.clone().formats);

    let qp_languages: String = utils::derive_query_params("languages", &set_languages);
    let qp_genres: String = utils::derive_query_params("genres", &set_genres);
    let qp_formats: String = utils::derive_query_params("formats", &set_formats);

    let all_genres: Vec<models::Genre> = serde_json::from_value(
        utils::fetch(endpoints.backend_url("/genres?order_by=genre"), &client)?
            .get("data")
            .cloned()
            .unwrap(),
    )
    .unwrap();
    let all_languages: Vec<models::Language> = serde_json::from_value(
        utils::fetch(
            endpoints.backend_url("/languages?order_by=language"),
            &client,
        )?
        .get("data")
        .cloned()
        .unwrap(),
    )
    .unwrap();
    let all_formats: Vec<models::Format> = serde_json::from_value(
        utils::fetch(endpoints.backend_url("/formats?order_by=format"), &client)?
            .get("data")
            .cloned()
            .unwrap(),
    )
    .unwrap();

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

            let qp_genres = utils::derive_query_params("genres", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres, qp_languages.clone(), qp_formats.clone()],
            );

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

            let qp_languages = utils::derive_query_params("languages", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres.clone(), qp_languages, qp_formats.clone()],
            );

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

            let qp_formats = utils::derive_query_params("formats", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres.clone(), qp_languages.clone(), qp_formats],
            );

            models::Filter {
                id,
                name: "format".to_string(),
                selected,
                value: format.format.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();

    let link = utils::derive_link("/titles", vec![qp_genres, qp_languages, qp_formats]);
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
