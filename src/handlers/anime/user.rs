use crate::AppServices;
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
// ---------------------- Structs -------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnime {
    watched: i64,
    score: f64,
    status: String,
}

#[async_trait]
pub trait UserAnimeActions {
    fn new(user_id: &String, anime_id: &String, base: UserAnime) -> Self
    where
        Self: Sized;
    async fn store_to_db(&self, service: web::Data<AppServices>) -> Result<String, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAnimeCreate {
    user_id: Thing,
    anime_id: Thing,
    #[serde(flatten)]
    base: UserAnime,
    updated_at: Datetime,
    created_at: Datetime,
}

#[async_trait]
impl UserAnimeActions for UserAnimeCreate {
    fn new(user_id: &String, anime_id: &String, base: UserAnime) -> UserAnimeCreate {
        UserAnimeCreate {
            user_id: Thing::from(("user", user_id.as_str())),
            anime_id: Thing::from(("anime", anime_id.as_str())),
            base,
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }
    }

    async fn store_to_db(&self, service: web::Data<AppServices>) -> Result<String, String> {
        let user_id = self.user_id.clone();
        let anime_id = self.anime_id.clone();
        let watched = self.base.watched;
        let score = self.base.score;
        let status = self.base.status.clone();
        let updated_at = self.updated_at.clone();
        let created_at = self.created_at.clone();

        let query_results: Result<Option<UserAnimeRecord>, surrealdb::Error> = match service
            .surreal
            .query(
                "
            RELATE $user_id->has_anime->$anime_id SET
            watched = $watched,
            score = $score,
            status = $status,
            updated_at = $updated_at,
            created_at = $created_at;",
            )
            .bind((
                "user_id",
                user_id,
                "anime_id",
                anime_id,
                "watched",
                watched,
                "score",
                score,
                "status",
                status,
                "updated_at",
                updated_at,
                "created_at",
                created_at,
            ))
            .await
        {
            Ok(mut data) => data.take(0),
            Err(e) => panic!("{}", e),
        };

        match query_results {
            Ok(Some(data)) => {
                serde_json::to_string(&data).map_err(|_| "Failed to Serialize Data".to_string())
            }
            Ok(None) => Err(format!("Failed to Create Record")),
            Err(e) => Err(format!("Database Error: {}", e)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserAnimeUpdate {
    user_id: Thing,
    anime_id: Thing,
    #[serde(flatten)]
    base: UserAnime,
    updated_at: Datetime,
}

#[async_trait]
impl UserAnimeActions for UserAnimeUpdate {
    fn new(user_id: &String, anime_id: &String, base: UserAnime) -> UserAnimeUpdate {
        UserAnimeUpdate {
            user_id: Thing::from(("user", user_id.as_str())),
            anime_id: Thing::from(("anime", anime_id.as_str())),
            base,
            updated_at: Datetime::default(),
        }
    }

    async fn store_to_db(&self, service: web::Data<AppServices>) -> Result<String, String> {
        let user_id = self.user_id.clone();
        let anime_id = self.anime_id.clone();
        let watched = self.base.watched;
        let score = self.base.score;
        let status = self.base.status.clone();
        let updated_at = self.updated_at.clone();

        let query_results: Result<Option<UserAnimeRecord>, surrealdb::Error> = match service
            .surreal
            .query(
                "UPDATE (SELECT id FROM has_anime WHERE in = $user_id AND out = $anime_id) SET score = $score, watched = $watched status = $status, updated_at = $updated_at;",
            ).bind((
                "user_id",user_id,
                "anime_id",anime_id,
                "watched",watched,
                "score",score,
                "status",status,
                "updated_at",updated_at,
            ))
            .await {
                Ok(mut data) => data.take(0),
                Err(e) => panic!("{}", e),
            };

        match query_results {
            Ok(Some(data)) => {
                serde_json::to_string(&data).map_err(|_| "Failed to Serialize Data".to_string())
            }
            Ok(None) => Err(format!("Failed to Update Record")),
            Err(e) => Err(format!("Database Error: {}", e)),
        }
    }
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

#[derive(Debug, Deserialize, Clone)]
struct FormData {
    id: String,
}

#[derive(Debug, Deserialize, Clone)]
struct FormDataFull {
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
    let param = Query::<FormDataFull>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("failed at query from params"));

    // log::info!("Params: {:?}", param.clone());
    // log::info!("Request: {:?}", req);

    let res = UserAnimeCreate::new(&param.uid, &param.aid, req.into_inner())
        .store_to_db(service)
        .await;

    match res {
        Ok(data) => HttpResponse::Created().body(data),
        Err(e) => HttpResponse::NotAcceptable().body(e),
    }
}

pub async fn handler_user_anime_patch(
    params: HttpRequest,
    req: web::Json<UserAnime>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormDataFull>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at query from params"));

    let res = UserAnimeUpdate::new(&param.uid, &param.aid, req.into_inner())
        .store_to_db(service)
        .await;

    match res {
        Ok(data) => HttpResponse::Ok().body(data),
        Err(e) => HttpResponse::NotAcceptable().body(e),
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
