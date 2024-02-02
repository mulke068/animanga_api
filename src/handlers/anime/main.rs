use crate::{middleware::caching::Caching, AppData};
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use surrealdb::sql::{Datetime, Thing};
// ---------------------- Structs -------------------

trait AnimeField {
    fn base(&self) -> Anime;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeNames {
    pub(super) original: String,
    pub(super) en: String,
    pub(super) jp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Anime {
    names: AnimeNames,

    season: u8,
    episodes: u8,
    score: f32,
    status: String,

    types: Vec<String>,
    platforms: Vec<String>,
    genres: Vec<String>,
    tags: Vec<String>,

    trailer_urls: Vec<String>,
    info_urls: Vec<String>,
    video_urls: Vec<String>,
    image_urls: Vec<String>,
}

impl AnimeField for Anime {
    fn base(&self) -> Anime {
        self.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct AnimeCreate {
    #[serde(flatten)]
    base: Anime,

    updated_at: Datetime,
    created_at: Datetime,
}

#[derive(Debug, Deserialize, Serialize)]
struct AnimeUpdate {
    #[serde(flatten)]
    base: Anime,

    updated_at: Datetime,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeRecord {
    id: Thing,

    #[serde(flatten)]
    base: Anime,

    updated_at: Datetime,
    created_at: Datetime,
}

#[derive(Deserialize)]
struct FormData {
    id: Option<String>,
}

// ---------------------------- Handlers ------------------------------

pub async fn handler_anime_get(req: HttpRequest, state: web::Data<AppData>) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to parse query string"));

    let cache = Caching::new(req.uri().to_string(), &state);

    match cache.get().await {
        Some(data) => {
            log::info!("Data Found in cache");

            // cache.timer_reset(&state).await;
            HttpResponse::Ok()
                .append_header((
                    "X-Cache-Remaining-Time",
                    cache.timer_get().await.to_string(),
                ))
                .body(data)
        }
        None => {
            log::info!("Data not found in cache");

            if let Some(id) = &param.id {
                let record: Option<AnimeRecord> = match state.surreal.select(("anime", id)).await {
                    Ok(data) => Some(data.unwrap()),
                    Err(_) => None,
                };

                let res: String = match &record {
                    Some(data) => {
                        let res = serde_json::to_string(&data)
                            .unwrap_or_else(|_| panic!("Failed to serialize data"));
                        cache.set(&res).await;
                        res
                    }
                    None => String::from("No data Found"),
                };

                return match &record {
                    Some(_) => HttpResponse::Ok().body(res),
                    None => HttpResponse::NotFound().body(res),
                };
            } else {
                let record: Vec<AnimeRecord> =
                    match state.surreal.query("SELECT * FROM anime").await {
                        Ok(mut data) => data.take(0).unwrap(),
                        Err(_) => Vec::new(),
                    };

                let res = serde_json::to_string(&record)
                    .unwrap_or_else(|_| panic!("Failed to format the data"));

                if String::from(&res) != "[]" {
                    cache.set(&res).await;
                    return HttpResponse::Ok().body(res);
                } else {
                    return HttpResponse::NotFound().body(res);
                }
            };
        }
    }
}

pub async fn handler_anime_post(
    payload: web::Json<Anime>,
    state: web::Data<AppData>,
) -> impl Responder {
    let record: Vec<AnimeRecord> = match state
        .surreal
        .create("anime")
        .content(AnimeCreate {
            base: payload.base(),
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        })
        .await
    {
        Ok(data) => data,
        Err(_) => Vec::new(),
    };

    let res: String = match &record.len() {
        1 => {
            let data = &record[0];
            // let index = state.meilisearch.index("anime");
            // let name: AnimeSearch = AnimeSearch {
            //     db_id: data.id.id.to_string(),
            //     original: data.base.names.original.clone(),
            //     en: data.base.names.en.clone(),
            //     jp: data.base.names.jp.clone(),
            // };
            // name.create(index).await;
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to format the data"))
        }
        _ => String::from("No Data found"),
    };

    if !&record.is_empty() {
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::NotAcceptable().body(res)
    }
}

pub async fn handler_anime_patch(
    req: HttpRequest,
    payload: web::Json<Anime>,
    state: web::Data<AppData>,
) -> impl Responder {
    let param: Query<FormData> = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed prase query to string"));

    if let Some(id) = &param.id {
        let record: Option<AnimeRecord> = match state
            .surreal
            .update(("anime", id))
            .merge(AnimeUpdate {
                base: payload.base(),
                updated_at: Datetime::default(),
            })
            .await
        {
            Ok(data) => data,
            Err(_) => None,
        };

        let res: String = match &record {
            Some(data) => {
                // let index = state.meilisearch.index("anime");
                // let name: AnimeSearch = AnimeSearch {
                //     db_id: data.id.id.to_string(),
                //     original: data.base.names.original.clone(),
                //     en: data.base.names.en.clone(),
                //     jp: data.base.names.jp.clone(),
                // };
                // name.update(index).await;
                serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to format the data"))
            }
            None => String::from("No Data found"),
        };

        if let Some(data) = Some(&res) {
            Caching::new(req.uri().to_string(), &state).set(data).await;
        }

        match &record {
            Some(_) => HttpResponse::Created().body(res),
            None => HttpResponse::NotFound().body(res),
        }
    } else {
        HttpResponse::BadRequest().body("Bad Request")
    }
}

pub async fn handler_anime_delete(req: HttpRequest, state: web::Data<AppData>) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to prase params to query"));

    if let Some(id) = &param.id {
        let record: Option<AnimeRecord> = state.surreal.delete(("anime", id)).await.unwrap();

        Caching::new(req.uri().to_string(), &state).delete().await;

        match &record {
            Some(_) => HttpResponse::Ok().body("Data Deleted"),
            None => HttpResponse::NotFound().body("No Data Found"),
        }
    } else {
        HttpResponse::BadRequest().body("Bad Request")
    }
}
