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

    let mut pages: Vec<models::Page> = (0..pagination.page_total)
        .map(|v| {
            let (qp_limit, qp_offset) =
                utils::derive_limit_offset(v > 0, pagination.limit, v * pagination.limit);

            let link = utils::derive_link(
                "/titles",
                vec![
                    qp_formats.clone(),
                    qp_genres.clone(),
                    qp_languages.clone(),
                    qp_limit,
                    qp_offset,
                ],
            );

            let page_number = v + 1;

            models::Page {
                active: pagination.page_current != page_number,
                link,
                number: page_number,
                selected: pagination.page_current == page_number,
                value: page_number.to_string(),
            }
        })
        .collect();

    if pages.len() > 3 {
        let mut pages_copy: Vec<models::Page> = pages.clone();
        let first = pages_copy.remove(0);
        let last = pages_copy.pop().unwrap();

        let inner_pages: Vec<models::Page> = pages_copy
            .iter()
            .cloned()
            .filter(|p| {
                let is_edge = pagination.page_current < 5
                    || pagination.page_current > pagination.page_total - 4;
                let offset_range = if is_edge { 4 } else { 3 };

                p.number > pagination.page_current - offset_range
                    && p.number < pagination.page_current + offset_range
            })
            .collect();

        let mut inner_pages_copy = inner_pages.clone();
        let first_2 = inner_pages_copy.remove(0);
        let last_2 = inner_pages_copy.pop().unwrap();

        let first_ellipsis: Vec<models::Page> = if first_2.number - first.number > 1 {
            vec![models::Page {
                active: false,
                link: "".to_string(),
                number: 2,
                selected: false,
                value: "...".to_string(),
            }]
        } else {
            vec![]
        };

        let second_ellipsis: Vec<models::Page> = if last.number - last_2.number > 1 {
            vec![models::Page {
                active: false,
                link: "".to_string(),
                number: 2,
                selected: false,
                value: "...".to_string(),
            }]
        } else {
            vec![]
        };

        pages = vec![
            vec![first],
            first_ellipsis,
            inner_pages,
            second_ellipsis,
            vec![last],
        ]
        .into_iter()
        .flatten()
        .collect();
    }

    let prev = models::PageControl {
        active: pagination.has_prev,
        link: match pagination.has_prev {
            true => {
                let (qp_limit, qp_offset) = utils::derive_limit_offset(
                    pagination.page_current != 2,
                    pagination.limit,
                    (pagination.page_current - 2) * pagination.limit,
                );

                let link = utils::derive_link(
                    "/titles",
                    vec![
                        qp_formats.clone(),
                        qp_genres.clone(),
                        qp_languages.clone(),
                        qp_limit,
                        qp_offset,
                    ],
                );

                link
            }
            false => "".to_string(),
        },
        value: "prev".to_string(),
    };

    let next = models::PageControl {
        active: pagination.has_next,
        link: match pagination.has_next {
            true => {
                let (qp_limit, qp_offset) = utils::derive_limit_offset(
                    true,
                    pagination.limit,
                    pagination.page_current * pagination.limit,
                );

                let link = utils::derive_link(
                    "/titles",
                    vec![
                        qp_formats.clone(),
                        qp_genres.clone(),
                        qp_languages.clone(),
                        qp_limit,
                        qp_offset,
                    ],
                );

                link
            }
            false => "".to_string(),
        },
        value: "next".to_string(),
    };

    let page_control_prev = vec![prev];
    let page_control_next = vec![next];

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

    let filter_genres = all_genres
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

    let filter_languages = all_languages
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

    let filter_formats = all_formats
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

    let data = serde_json::json!({
        "assets": endpoints.assets,
        "formats": serde_json::json!(filter_formats),
        "genres": serde_json::json!(filter_genres),
        "languages": serde_json::json!(filter_languages),
        "page_control_next": serde_json::json!(page_control_next),
        "page_control_prev": serde_json::json!(page_control_prev),
        "pages": serde_json::json!(pages),
        "titles": titles["data"],
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}
