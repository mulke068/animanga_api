use crate::{middleware::caching::Caching, AppServices};
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use surrealdb::sql::{Datetime, Thing};

// ---------------------- Structs -------------------

trait MangaField {
    fn base(&self) -> Manga;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MangaNames {
    pub(super) original: String,
    pub(super) en: String,
    pub(super) jp: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Manga {
    names: MangaNames,
    chapters: u16,
    volumes: u16,
    score: f32,
    status: String,
    types: Vec<String>,
    platforms: Vec<String>,
    genres: Vec<String>,
    tags: Vec<String>,
    info_urls: Vec<String>,
    read_urls: Vec<String>,
    image_urls: Vec<String>,
}

impl MangaField for Manga {
    fn base(&self) -> Manga {
        self.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct MangaCreate {
    #[serde(flatten)]
    base: Manga,
    updated_at: Datetime,
    created_at: Datetime,
}
#[derive(Debug, Deserialize, Serialize)]
struct MangaUpdate {
    #[serde(flatten)]
    base: Manga,
    updated_at: Datetime,
}
#[derive(Debug, Deserialize, Serialize)]
struct MangaRecord {
    id: Thing,
    #[serde(flatten)]
    base: Manga,
    updated_at: Datetime,
    created_at: Datetime,
}

#[derive(Deserialize)]
struct FormData {
    id: Option<String>,
}

// ---------------------------- Handlers ------------------------------

pub async fn handler_manga_get(req: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to query from params"));

    let cache = Caching::new(req.uri().to_string(), &service);

    match cache.get().await {
        Some(data) => {
            // cache.timer_reset(&service).await;
            HttpResponse::Ok()
                .append_header((
                    "X-Cache-Remaining-Time",
                    cache.timer_get().await.to_string(),
                ))
                .body(data)
        }
        None => {
            if let Some(id) = &param.id {
                let record: Option<MangaRecord> = match service.surreal.select(("manga", id)).await {
                    Ok(data) => data,
                    Err(_) => None,
                };

                let res: String = match &record {
                    Some(data) => {
                        let res = serde_json::to_string(&data)
                            .unwrap_or_else(|_| panic!("Failed at Serialize data"));
                        cache.set(&res).await;
                        res
                    }
                    None => String::from("No Data Found"),
                };

                match &record {
                    Some(_) => HttpResponse::Ok().body(res),
                    None => HttpResponse::NotFound().body(res),
                }
            } else {
                let record: Vec<MangaRecord> =
                    match service.surreal.query("SELECT * FROM manga").await {
                        Ok(mut data) => data.take(0).unwrap(),
                        Err(_) => Vec::new(),
                    };

                let res: String = match &record.len() {
                    0 => String::from("No Data Found"),
                    _ => {
                        let res = serde_json::to_string(&record)
                            .unwrap_or_else(|_| panic!("Failed at Serialize data"));
                        cache.set(&res).await;
                        res
                    }
                };

                match &record.len() {
                    0 => HttpResponse::NotFound().body(res),
                    _ => HttpResponse::Ok().body(res),
                }
            }
        }
    }
}

pub async fn handler_manga_post(
    payload: web::Json<Manga>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let record: Vec<MangaRecord> = match service
        .surreal
        .create("manga")
        .content(MangaCreate {
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
            // let search_index = service.meilisearch.index("manga");
            // let names: MangaSearch = MangaSearch {
            //     db_id: data.id.id.to_string(),
            //     original: data.base.names.original.clone(),
            //     en: data.base.names.en.clone(),
            //     jp: data.base.names.jp.clone(),
            // };
            // names.create(search_index).await;
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed at Serialize data"))
        }
        _ => String::from("Failed to Create"),
    };

    if !&record.is_empty() {
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::NotAcceptable().body(res)
    }
}

pub async fn handler_manga_patch(
    req: HttpRequest,
    payload: web::Json<Manga>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to query from params"));

    if let Some(id) = &param.id {
        let record: Option<MangaRecord> = match service
            .surreal
            .update(("manga", id))
            .merge(MangaUpdate {
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
                // let search_index = service.meilisearch.index("manga");
                // let names: MangaSearch = MangaSearch {
                //     db_id: data.id.id.to_string(),
                //     original: data.base.names.original.clone(),
                //     en: data.base.names.en.clone(),
                //     jp: data.base.names.jp.clone(),
                // };
                // names.create(search_index).await;
                serde_json::to_string(&data)
                    .unwrap_or_else(|_| panic!("Failed to Serialize to String"))
            }
            None => String::from("No Data Found | Failed to Create"),
        };

        if let Some(data) = Some(&res) {
            Caching::new(req.uri().to_string(), &service).set(data).await;
        }

        match &record {
            Some(_) => HttpResponse::Created().body(res),
            None => HttpResponse::NotFound().body(res),
        }
    } else {
        HttpResponse::BadRequest().body("No ID Provided")
    }
}

pub async fn handler_manga_delete(req: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to query from params"));

    if let Some(id) = &param.id {
        let record: Option<MangaRecord> = match service.surreal.delete(("manga", id)).await {
            Ok(data) => data,
            Err(_) => None,
        };

        Caching::new(req.uri().to_string(), &service).delete().await;

        match &record {
            Some(_) => HttpResponse::Ok().body("Data Deleted"),
            None => HttpResponse::NotFound().body("No Data Found"),
        }
    } else {
        HttpResponse::BadRequest().body("No ID Provided")
    }
}
