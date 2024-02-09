use crate::AppServices;
// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Id, Thing};
// ---------------------- Structs -------------------

trait UserMangaField {
    fn base(&self) -> UserManga;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserManga {
    read: u16,
    score: u8,
    status: String,
}

impl UserMangaField for UserManga {
    fn base(&self) -> UserManga {
        self.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct UserMangaCreate {
    user_id: Thing,
    manga_id: Thing,
    #[serde(flatten)]
    base: UserManga,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserMangaUpdate {
    #[serde(flatten)]
    base: UserManga,
    updated_at: Datetime,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserMangaRecord {
    id: Thing,
    #[serde(rename = "in")]
    input: Thing,
    #[serde(rename = "out")]
    output: Thing,
    #[serde(flatten)]
    base: UserManga,
    updated_at: Datetime,
    created_at: Datetime,
}

#[derive(Deserialize)]
struct FormData {
    id: String,
}

#[derive(Deserialize)]
struct FormDataCreated {
    uid: String,
    mid: String,
}

// ---------------------- Handlers -------------------

pub async fn handler_user_manga_get(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query the data"));

    let record: Option<UserMangaRecord> =
        match service.surreal.select(("user_manga", &param.id)).await {
            Ok(data) => data,
            Err(e) => panic!("{}", e),
        };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed at Serialize data"))
        }
        None => String::from("No Data Found"),
    };

    match &record {
        Some(_) => HttpResponse::Ok().body(res),
        None => HttpResponse::NotFound().body(res),
    }
}

pub async fn handler_user_manga_post(
    params: HttpRequest,
    req: web::Json<UserManga>,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormDataCreated>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at parsing data"));

    let record: Option<UserMangaRecord> = match service
        .surreal
        .query(
            "RELATE $user_id->user_manga->$manga_id SET 
            read = $read, 
            score = $score, 
            status = $status, 
            updated_at = time::now(), 
            created_at = time::now();",
        )
        .bind(UserMangaCreate {
            user_id: Thing {
                tb: String::from("user"),
                id: Id::String(param.uid.clone()),
            },
            manga_id: Thing {
                tb: String::from("manga"),
                id: Id::String(param.mid.clone()),
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
        None => String::from("No Data Created"),
    };

    match &record {
        Some(_) => HttpResponse::Created().body(res),
        None => HttpResponse::NotAcceptable().body(res),
    }
}

pub async fn handler_user_manga_patch(
    req: web::Json<UserManga>,
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at parsing data"));

    let record: Option<UserMangaRecord> = match service
        .surreal
        .update(("user_manga", &param.id))
        .merge(UserMangaUpdate {
            base: req.base(),
            updated_at: Datetime::default(),
        })
        .await
    {
        Ok(data) => data,
        Err(e) => panic!("{}", e),
    };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed at Serialize data"))
        }
        None => String::from("No Data Created"),
    };

    match &record {
        Some(_) => HttpResponse::Created().body(res),
        None => HttpResponse::NotAcceptable().body(res),
    }
}

pub async fn handler_user_manga_delete(
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed at parsing data"));

    let record: Option<UserMangaRecord> =
        match service.surreal.delete(("user_manga", &param.id)).await {
            Ok(data) => data,
            Err(e) => panic!("{}", e),
        };

    match &record {
        Some(_) => HttpResponse::Ok().body("Data Deleted"),
        None => HttpResponse::NotFound().body("No Data Found"),
    }
}
