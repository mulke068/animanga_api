use crate::AppServices;
// ---------------------------- Imports ------------------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::Deserialize;
use surrealdb::sql::Datetime;


use super::user_structs::{Users, UsersField, UsersRecord, UsersUpdate};
// ---------------------------- Structs ------------------------------

#[derive(Deserialize)]
struct FormData {
    uid: String,
}

// ---------------------------- Handlers ------------------------------
pub async fn handler_user_get(params: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query params"));

    let record: Option<UsersRecord> = match service.surreal.select(("user", &param.uid)).await {
        Ok(data) => data,
        Err(_) => None,
    };

    let res: String = match &record {
        Some(data) => {
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to prase to string"))
        }
        None => String::from("No Data Found"),
    };

    match &record {
        Some(_) => HttpResponse::Ok().body(res),
        None => HttpResponse::NotFound().body(res),
    }
}

pub async fn handler_user_post(req: web::Json<Users>, service: web::Data<AppServices>) -> impl Responder {

    let content = req.clone();

    let record: Vec<UsersRecord> = match service
        .surreal
        .create("user")
        .content(content)
        .await
    {
        Ok(record) => record.unwrap_or(Vec::new()),
        Err(_) => Vec::new(),
    };

    let res: String = match &record.len() {
        1 => {
            let data = &record[0];
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to prase to string"))
        }
        _ => String::from("No Data Found"),
    };

    if !&record.is_empty() {
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::NotAcceptable().body(res)
    }
}

pub async fn handler_user_patch(
    req: web::Json<Users>,
    params: HttpRequest,
    service: web::Data<AppServices>,
) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query params"));

    let record: Option<UsersRecord> = match service
        .surreal
        .update(("user", &param.uid))
        .merge(UsersUpdate {
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
            serde_json::to_string(&data).unwrap_or_else(|_| panic!("Failed to prase to string"))
        }
        None => String::from("No Data Found"),
    };

    match &record {
        Some(_) => HttpResponse::Created().body(res),
        None => HttpResponse::NotFound().body(res),
    }
}

pub async fn handler_user_delete(params: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let param = Query::<FormData>::from_query(&params.query_string())
        .unwrap_or_else(|_| panic!("Failed to query params"));

    let record: Option<UsersRecord> = match service.surreal.delete(("user", &param.uid)).await {
        Ok(data) => data,
        Err(_) => None,
    };

    match &record {
        Some(_) => HttpResponse::Ok().body("Data Deleted"),
        None => HttpResponse::NotFound().body("No Data Found"),
    }
}
