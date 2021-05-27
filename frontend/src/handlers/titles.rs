use crate::{errors, models, querystrings, utils};
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

pub async fn all(
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::Endpoints>,
    web::Query(filter_qs): web::Query<querystrings::Filters>,
    web::Query(order_by): web::Query<querystrings::Order>,
) -> Result<HttpResponse, errors::MyError> {
    let set_genres = utils::ids_set(filter_qs.clone().genres);
    let set_languages = utils::ids_set(filter_qs.clone().languages);
    let set_formats = utils::ids_set(filter_qs.clone().formats);

    let qp_languages: String = utils::derive_query_params("languages", &set_languages);
    let qp_genres: String = utils::derive_query_params("genres", &set_genres);
    let qp_formats: String = utils::derive_query_params("formats", &set_formats);
    let qp_offset: String = order_by
        .offset
        .map(|value| Some(value.to_string()))
        .map(utils::ids_set)
        .as_ref()
        .map(|value| utils::derive_query_params("offset", value))
        .unwrap_or("offset=0".to_string());
    let qp_limit: String = order_by
        .limit
        .map(|value| Some(value.to_string()))
        .map(utils::ids_set)
        .as_ref()
        .map(|value| utils::derive_query_params("limit", value))
        .unwrap_or("limit=25".to_string());

    let titles_link = utils::derive_link(
        "/titles",
        vec![
            qp_formats.clone(),
            qp_genres.clone(),
            qp_languages.clone(),
            qp_limit,
            qp_offset,
        ],
    );
    let titles = utils::fetch(endpoints.backend_url(&titles_link), &client)?;

    let pagination = titles
        .get("pagination")
        .cloned()
        .map(serde_json::from_value::<models::Pagination>)
        .unwrap_or(Ok(models::Pagination::default()))
        .unwrap();

    let pages = utils::derive_pages(
        pagination.items_total,
        pagination.limit,
        pagination.page_current,
        pagination.page_total,
        vec![qp_formats.clone(), qp_genres.clone(), qp_languages.clone()],
    );

    let (page_control_prev, page_control_next) = utils::derive_page_controls(
        pagination.has_next,
        pagination.has_prev,
        pagination.limit,
        pagination.page_current,
        vec![qp_formats.clone(), qp_genres.clone(), qp_languages.clone()],
    );

    let all_genres: Vec<models::Genre> =
        utils::fetch(endpoints.backend_url("/genres?order_by=genre"), &client)?
            .get("data")
            .cloned()
            .map(serde_json::from_value)
            .unwrap()
            .unwrap_or(vec![]);

    let all_languages: Vec<models::Language> = utils::fetch(
        endpoints.backend_url("/languages?order_by=language"),
        &client,
    )?
    .get("data")
    .cloned()
    .map(serde_json::from_value)
    .unwrap()
    .unwrap_or(vec![]);

    let all_formats: Vec<models::Format> =
        utils::fetch(endpoints.backend_url("/formats?order_by=format"), &client)?
            .get("data")
            .cloned()
            .map(serde_json::from_value)
            .unwrap()
            .unwrap_or(vec![]);

    let mut filter_genres = all_genres
        .iter()
        .map(|genre| {
            let id = genre.id.unwrap();
            let set = utils::get_sym_diff(id, &set_genres);
            let qp_genres = utils::derive_query_params("genres", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres, qp_languages.clone(), qp_formats.clone()],
            );

            models::Filter {
                id,
                name: "genre".to_string(),
                selected: !set.contains(&id),
                value: genre.genre.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();
    filter_genres.sort_by(|a, b| b.selected.partial_cmp(&a.selected).unwrap());

    let mut filter_languages = all_languages
        .iter()
        .map(|language| {
            let id = language.id.unwrap();
            let set = utils::get_sym_diff(id, &set_languages);
            let qp_languages = utils::derive_query_params("languages", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres.clone(), qp_languages, qp_formats.clone()],
            );

            models::Filter {
                id,
                name: "language".to_string(),
                selected: !set.contains(&id),
                value: language.language.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();
    filter_languages.sort_by(|a, b| b.selected.partial_cmp(&a.selected).unwrap());

    let mut filter_formats = all_formats
        .iter()
        .map(|format| {
            let id = format.id.unwrap();
            let set = utils::get_sym_diff(id, &set_formats);
            let qp_formats = utils::derive_query_params("formats", &set);
            let link = utils::derive_link(
                "/titles",
                vec![qp_genres.clone(), qp_languages.clone(), qp_formats],
            );

            models::Filter {
                id,
                name: "format".to_string(),
                selected: !set.contains(&id),
                value: format.format.clone(),
                link,
            }
        })
        .collect::<Vec<models::Filter>>();
    filter_formats.sort_by(|a, b| b.selected.partial_cmp(&a.selected).unwrap());

    let filter_tags: Vec<models::Filter> = vec![
        filter_genres.clone(),
        filter_languages.clone(),
        filter_formats.clone(),
    ]
    .into_iter()
    .flatten()
    .filter(|f| f.selected)
    .collect();

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "filters": serde_json::json!({
            "formats": serde_json::json!(filter_formats),
            "genres": serde_json::json!(filter_genres),
            "languages": serde_json::json!(filter_languages),
            "tags": serde_json::json!(filter_tags),
        }),
        "pagination": serde_json::json!({
            "next": serde_json::json!(page_control_next),
            "prev": serde_json::json!(page_control_prev),
            "pages": serde_json::json!(pages),
            "page_total": serde_json::json!(pagination.page_total),
        }),
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
