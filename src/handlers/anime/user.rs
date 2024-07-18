use crate::AppServices;
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
// ---------------------- Structs -------------------

trait UserAnimeField {
    fn base(&self) -> UserAnime;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAnime {
    watched: i64,
    score: f64,
    status: String,
}

impl UserAnimeField for UserAnime {
    fn base(&self) -> UserAnime {
        self.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAnimeCreate {
    user_id: Thing,
    anime_id: Thing,
    #[serde(flatten)]
    base: UserAnime,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAnimeUpdate {
    #[serde(flatten)]
    base: UserAnime,
    updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAnimeRecord {
    id: Thing,
    #[serde(rename = "in")]
    input: Thing,
    #[serde(rename = "out")]
    output: Thing,
    #[serde(flatten)]
    base: UserAnime,
    updated_at: Datetime,
    created_at: Datetime,
}

#[derive(Deserialize)]
struct FormData {
    id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct FormDataCreated {
    uid: String,
    aid: String,
}
// ---------------------- Handlers -------------------

pub async fn handler_user_anime_get(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at query from params"));

    let record: Option<UserAnimeRecord> =
        match service.surreal.select(("has_anime", &param.id)).await {
            Ok(data) => data,
            Err(e) => panic!("{}", e),
        };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to Serialize data"))
        }
        None => String::from("No Data Found"),
    };

    match &record {
        Some(_) => HttpResponse::Ok().body(res),
        None => HttpResponse::NotFound().body(res),
    }
}

pub async fn handler_user_anime_post(
    params: HttpRequest,
    req: web::Json<UserAnime>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormDataCreated>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("failed at query from params"));

    log::info!("Params: {:?}", param.clone());
    log::info!("Request: {:?}", req);

    let record: Option<UserAnimeRecord> = match service
        .surreal
        .query(
            "RELATE $user_id->has_anime->$anime_id SET 
            watched = $watched, 
            score = $score, 
            status = $status, 
            updated_at = time::now(), 
            created_at = time::now();",
        )
        .bind(UserAnimeCreate {
            user_id: Thing {
                tb: String::from("user"),
                id: Id::String(param.uid.clone()),
            },
            anime_id: Thing {
                tb: String::from("anime"),
                id: Id::String(param.aid.clone()),
            },
            base: req.base(),
        })
        .await
    {
        Ok(mut data) => data.take(0).unwrap(),
        Err(e) => panic!("{}", e),
    };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed at Serialize data"))
        }
        None => String::from("Failed to Create"),
    };

    match &record {
        Some(_) => HttpResponse::Created().body(res),
        None => HttpResponse::NotAcceptable().body(res),
    }
}

pub async fn handler_user_anime_patch(
    params: HttpRequest,
    req: web::Json<UserAnime>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at query from params"));

    let record: Option<UserAnimeRecord> = match service
        .surreal
        .update(("has_anime", &param.id))
        .merge(UserAnimeUpdate {
            base: req.base(),
            updated_at: Datetime::default(),
        })
        .await
    {
        Ok(data) => data,
        Err(_) => None,
    };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to Serialize data"))
        }
        None => String::from("Failed to Update"),
    };

    match &record {
        Some(_) => HttpResponse::Created().body(res),
        None => HttpResponse::NotAcceptable().body(res),
    }
}

pub async fn handler_user_anime_delete(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query from Params"));

    let record: Option<UserAnimeRecord> =
        match service.surreal.delete(("has_anime", &param.id)).await {
            Ok(data) => data,
            Err(_) => None,
        };

    match &record {
        Some(_) => HttpResponse::Ok().body("Data Deleted"),
        None => HttpResponse::NotFound().body("No Data Found"),
    }
}
