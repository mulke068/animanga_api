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
use chrono::{DateTime, Utc};
use log::info;

pub async fn handler_anime_get(
    req: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    // Capture the start time
    let start_time = Utc::now();

    // Step 1: Parse query string
    let step1_start = Utc::now();
    let param = Query::<FormData>::from_query(&req.query_string())
        .unwrap_or_else(|_| panic!("Failed to parse query string"));
    let step1_end = Utc::now();

    // Step 2: Create cache object
    let step2_start = step1_end;
    let cache = Caching::new(req.uri().to_string(), &service);
    let step2_end = Utc::now();

    // Step 3: Check cache
    let step3_start = step2_end;
    let cache_data = cache.get().await;
    let step3_end = Utc::now();

    // Initialize variables for logging
    let mut step4_start: DateTime<Utc> = Utc::now();
    let mut step4_end: DateTime<Utc> = Utc::now();
    let mut step5_start: DateTime<Utc> = Utc::now();
    let mut step5_end: DateTime<Utc> = Utc::now();
    let mut step6_start: DateTime<Utc> = Utc::now();
    let mut step6_end: DateTime<Utc> = Utc::now();
    let mut step7_start: DateTime<Utc> = Utc::now();
    let mut step7_end: DateTime<Utc> = Utc::now();
    let mut step8_start: DateTime<Utc> = Utc::now();
    let mut step8_end: DateTime<Utc> = Utc::now();
    let mut step9_start: DateTime<Utc> = Utc::now();
    let mut step9_end: DateTime<Utc> = Utc::now();
    let mut step10_start: DateTime<Utc> = Utc::now();
    let mut step10_end: DateTime<Utc> = Utc::now();


    let response = match cache_data.clone() {
        Some(data) => {
            info!("Data Found in cache");
    
            // Step 4: Prepare cached response
            step4_start = step3_end;
            let response = HttpResponse::Ok()
                .append_header((
                    "X-Cache-Remaining-Time",
                    cache.timer_get().await.to_string(),
                ))
                .body(data);
            step4_end = Utc::now();
    
            response
        }
        None => {
            info!("Data not found in cache");


            // Step 5: Retrieve data from database
            step5_start = step3_end;
            let response = if let Some(id) = &param.id {
                let record: Option<AnimeRecord> = match service.surreal.select(("anime", id)).await {
                    Ok(data) => Some(data.unwrap()),
                    Err(_) => None,
                };
                step5_end = Utc::now();
    
                // Step 6: Serialize data and update cache
                step6_start = step5_end;
                let res: String = match &record {
                    Some(data) => {
                        let res = serde_json::to_string(&data)
                            .unwrap_or_else(|_| panic!("Failed to serialize data"));
                        cache.set(&res).await;
                        res
                    }
                    None => String::from("No data Found"),
                };
                step6_end = Utc::now();
    
                // Step 7: Prepare response
                step7_start = step6_end;
                let response = match &record {
                    Some(_) => HttpResponse::Ok().body(res),
                    None => HttpResponse::NotFound().body(res),
                };
                step7_end = Utc::now();
    
                response
            } else {
                // Step 8: Query all records
                step8_start = step5_start;
                let record: Vec<AnimeRecord> =
                    match service.surreal.query("SELECT * FROM anime").await {
                        Ok(mut data) => data.take(0).unwrap(),
                        Err(_) => Vec::new(),
                    };
                step8_end = Utc::now();
    
                // Step 9: Serialize data and update cache
                step9_start = step8_end;
                let res = serde_json::to_string(&record)
                    .unwrap_or_else(|_| panic!("Failed to format the data"));
                if res != "[]" {
                    cache.set(&res).await;
                    step9_end = Utc::now();
    
                    // Step 10: Prepare response
                    step10_start = step9_end;
                    let response = HttpResponse::Ok().body(res);
                    step10_end = Utc::now();
    
                    response
                } else {
                    step10_start = step9_start;
                    let response = HttpResponse::NotFound().body(res);
                    step10_end = step10_start;
    
                    response
                }
            };
            response
        }
    };

    // Capture the end time and calculate the total duration
    let end_time = Utc::now();

    // Log durations
    info!("Step 1 (Parse query string): {} ms", (step1_end - step1_start).num_milliseconds());
    info!("Step 2 (Create cache object): {} ms", (step2_end - step2_start).num_milliseconds());
    info!("Step 3 (Check cache): {} ms", (step3_end - step3_start).num_milliseconds());
    if let Some(_) = cache_data {
        info!("Step 4 (Prepare cached response): {} ms", (step4_end - step4_start).num_milliseconds());
    }
    if let None = cache_data {
        info!("Step 5 (Retrieve data from database): {} ms", (step5_end - step5_start).num_milliseconds());
        info!("Step 6 (Serialize data and update cache): {} ms", (step6_end - step6_start).num_milliseconds());
        info!("Step 7 (Prepare response): {} ms", (step7_end - step7_start).num_milliseconds());
        info!("Step 8 (Query all records): {} ms", (step8_end - step8_start).num_milliseconds());
        info!("Step 9 (Serialize data and update cache): {} ms", (step9_end - step9_start).num_milliseconds());
        info!("Step 10 (Prepare response): {} ms", (step10_end - step10_start).num_milliseconds());
    }
    info!("Total request processing time: {} ms", (end_time - start_time).num_milliseconds());


    response
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
        Ok(data) => data.unwrap_or(Vec::new()),
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


pub async fn handler_anime_multi_post(req: web::Json<Vec<Anime>>, services: web::Data<AppServices>) -> impl Responder {
    let mut counter: usize = 0;
    let mut error_counter: usize = 0;

    for anime in req.iter() {
        let _record: Vec<AnimeRecord> = match services.surreal.create("anime").content(AnimeCreate {
            base: anime.base(),
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }).await {
            Ok(data) => data.unwrap_or(Vec::new()),
            Err(e) => {
                log::error!("Error At Surrealdb: {:?}", e);
                error_counter += 1;
                Vec::new()
            },
        };
        counter += 1;
    }

    {
        log::info!("{} Record Count", counter);
        log::info!("{} Records created", counter - error_counter);
        log::info!("{} Records failed", error_counter);
    }

    if counter == req.len() {
        let res = format!("{} records created", counter);
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::InternalServerError().finish()
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
