use crate::{errors, models, querystrings, utils};
use actix_http::Response;
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use reqwest::blocking::Client;

pub async fn all(
    req: HttpRequest,
    hb: web::Data<Handlebars<'_>>,
    client: web::Data<Client>,
    endpoints: web::Data<models::Endpoints>,
    web::Query(filter_qs): web::Query<querystrings::Filters>,
    web::Query(order_by): web::Query<querystrings::Order>,
) -> Result<HttpResponse, errors::MyError> {
    let is_mobile = utils::is_mobile_user_agent(req);

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

    let mut filter_tags: Vec<models::Filter> = vec![
        filter_genres.clone(),
        filter_languages.clone(),
        filter_formats.clone(),
    ]
    .into_iter()
    .flatten()
    .filter(|f| f.selected)
    .collect();
    utils::add_remove_all(&mut filter_tags);

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
        "is_mobile": is_mobile,
    });

    let body = hb.render("pages/titles", &data).unwrap();
    Ok(HttpResponse::Ok().body(body))
}

pub async fn one(
    hb: web::Data<Handlebars<'_>>,
    web::Path(id): web::Path<i64>,
    client: web::Data<Client>,
    endpoints: web::Data<models::Endpoints>,
) -> Result<HttpResponse, errors::MyError> {
    let link = format!("/titles/{}", id);
    let res_title = utils::fetch(endpoints.backend_url(&link), &client)?;

    match res_title.get("error").cloned() {
        Some(_) => Ok(Response::build(StatusCode::NOT_FOUND).finish()),
        None => {
            let title: models::Title = res_title
                .get("data")
                .cloned()
                .map(serde_json::from_value)
                .unwrap()
                .unwrap();

            let measure: models::Measure =
                utils::fetch(endpoints.backend_url(&format!("/measures/{}", id)), &client)?
                    .get("data")
                    .cloned()
                    .map(serde_json::from_value)
                    .unwrap()
                    .unwrap();

            let genre = &utils::fetch(
                // TODO: https://github.com/rodmoioliveira/livs-stack/issues/2
                endpoints.backend_url(&format!("/genres/{}", title.genre)),
                &client,
            )?
            .get("data")
            .cloned()
            .unwrap()["genre"];

            let format = &utils::fetch(
                // TODO: https://github.com/rodmoioliveira/livs-stack/issues/2
                endpoints.backend_url(&format!("/formats/{}", title.format)),
                &client,
            )?
            .get("data")
            .cloned()
            .unwrap()["format"];

            let language = &utils::fetch(
                // TODO: https://github.com/rodmoioliveira/livs-stack/issues/2
                endpoints.backend_url(&format!("/languages/{}", title.language)),
                &client,
            )?
            .get("data")
            .cloned()
            .unwrap()["language"];

            let publisher = &utils::fetch(
                // TODO: https://github.com/rodmoioliveira/livs-stack/issues/2
                endpoints.backend_url(&format!("/publishers/{}", title.publisher)),
                &client,
            )?
            .get("data")
            .cloned()
            .unwrap()["publisher"];

            let author = utils::fetch(
                // TODO: https://github.com/rodmoioliveira/livs-stack/issues/2
                endpoints.backend_url(&format!("/authors/{}", title.author)),
                &client,
            )?
            .get("data")
            .cloned()
            .unwrap();

            let data = serde_json::json!({
                "assets": endpoints.assets,
                "title": serde_json::json!({
                    "author": format!("{} {}", &author["first_name"], &author["last_name"]).replace("\"", ""),
                    "cover": title.cover,
                    "edition": title.edition,
                    "format": format,
                    "genre": genre,
                    "isbn": title.isbn,
                    "language": language,
                    "pages": title.pages,
                    "publisher": publisher,
                    "summary": title.summary,
                    "title": title.title,
                    "year": title.year,
                }),
                "measure": measure,
            });

            let body = hb.render("pages/title", &data).unwrap();
            Ok(HttpResponse::Ok().body(body))
        }
    }
}
