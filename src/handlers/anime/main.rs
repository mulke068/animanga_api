use crate::{middleware::caching::Caching, AppServices};
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};

use surrealdb::sql::{Datetime, Thing};
// ---------------------- Structs -------------------

pub trait AnimeField {
    fn base(&self) -> Anime;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnimeNames {
    pub original: String,
    pub en: Option<String>,
    pub jp: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Anime {
    pub names: AnimeNames,

    pub season: i64,
    pub episodes: i64,
    pub score: f64,
    pub status: String,

    pub types: Vec<String>,
    pub platforms: Vec<String>,
    pub genres: Vec<String>,
    pub tags: Vec<String>,

    pub trailer_urls: Vec<String>,
    pub info_urls: Vec<String>,
    pub video_urls: Vec<String>,
    pub image_urls: Vec<String>,
}

impl AnimeField for Anime {
    fn base(&self) -> Anime {
        self.clone()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeCreate {
    #[serde(flatten)]
    pub base: Anime,

    pub updated_at: Datetime,
    pub created_at: Datetime,
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

pub async fn handler_anime_get(
    req: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to parse query string"));

    let cache = Caching::new(req.uri().to_string(), &service);

    match cache.get().await {
        Some(data) => {
            log::info!("Data Found in cache");

            // cache.timer_reset(&service).await;
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
                let record: Option<AnimeRecord> = match service.surreal.select(("anime", id)).await
                {
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
                    match service.surreal.query("SELECT * FROM anime").await {
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
    service: web::Data<AppServices>,
) -> impl Responder {
    let record: Vec<AnimeRecord> = match service
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
            // let index = service.meilisearch.index("anime");
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
        Caching::new(String::from("/anime"), &service)
            .delete()
            .await;
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::NotAcceptable().body(res)
    }
}

pub async fn handler_anime_patch(
    req: HttpRequest,
    payload: web::Json<Anime>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param: Query<FormData> = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed prase query to string"));

    if let Some(id) = &param.id {
        let record: Option<AnimeRecord> = match service
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
                // let index = service.meilisearch.index("anime");
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
            Caching::new(req.uri().to_string(), &service)
                .set(data)
                .await;
        }

        match &record {
            Some(_) => HttpResponse::Created().body(res),
            None => HttpResponse::NotFound().body(res),
        }
    } else {
        HttpResponse::BadRequest().body("Bad Request")
    }
}

pub async fn handler_anime_delete(
    req: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to prase params to query"));

    if let Some(id) = &param.id {
        if let Ok(_data) = service
            .surreal
            .delete::<Option<AnimeRecord>>(("anime", id))
            .await
        {
            Caching::new(req.uri().to_string(), &service).delete().await;
            return HttpResponse::Ok().body("Data Deleted");
        } else {
            return HttpResponse::NotFound().body("No Data Found");
        }
    } else {
        return HttpResponse::BadRequest().body("Bad Request");
    }

    // if let Some(id) = &param.id {
    //     match service.surreal.delete::<Option<AnimeRecord>>(("anime", id)).await {
    //         Ok(record) => {
    //             Caching::new(req.uri().to_string(), &service).delete().await;

    //             match record {
    //                 Some(_) => HttpResponse::Ok().body("Data Deleted"),
    //                 None => HttpResponse::NotFound().body("No Data Found"),
    //             }
    //         }
    //         Err(e) => {
    //             // Log the error and return a response indicating that an error occurred
    //             log::error!("Failed to delete record: {}", e);
    //             HttpResponse::InternalServerError().body("Internal Server Error")
    //         }
    //     }
    // } else {
    //     HttpResponse::BadRequest().body("Bad Request")
    // }
}
