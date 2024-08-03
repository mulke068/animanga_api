use crate::AppServices;
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
// ---------------------- Structs -------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserAnime {
    watched: i64,
    score: f64,
    status: String,
}

trait UserAnimeField {
    fn base(&self) -> UserAnime;
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
    updated_at: Datetime,
    created_at: Datetime,
}

trait UserAnimeActions {
    fn new(user_id: &String, anime_id: &String, base: UserAnime) -> UserAnimeCreate;
    async fn create_to_db(&self, service: web::Data<AppServices>) -> Result<String, String>;
}

impl UserAnimeActions for UserAnimeCreate {
    fn new(user_id: &String, anime_id: &String, base: UserAnime) -> UserAnimeCreate {
        UserAnimeCreate {
            user_id: Thing {
                tb: String::from("user"),
                id: Id::String(user_id.clone()),
            },
            anime_id: Thing {
                tb: String::from("anime"),
                id: Id::String(anime_id.clone()),
            },
            base,
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }
    }
    async fn create_to_db(&self, service: web::Data<AppServices>) -> Result<String, String> {
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
            .bind(self)
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

    // log::info!("Params: {:?}", param.clone());
    // log::info!("Request: {:?}", req);

    let res = UserAnimeCreate::new(&param.uid, &param.aid, req.base())
        .create_to_db(service)
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
