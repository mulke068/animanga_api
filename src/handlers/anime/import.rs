use actix_web::{web, HttpResponse, Responder};
use surrealdb::sql::Datetime;

use crate::AppServices;

use super::main::*;

pub async fn post(req: web::Json<Vec<Anime>>, services: web::Data<AppServices>) -> impl Responder {
    let mut counter: usize = 0;

    for anime in req.iter() {
        let _record: Vec<AnimeRecord> = match services.surreal.create("anime").content(AnimeCreate {
            base: anime.base(),
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }).await {
            Ok(data) => data,
            Err(_) => Vec::new(),
        };
        counter += 1;
    }

    {
        log::info!("{} records created", counter);
    }

    if counter == req.len() {
        let res = format!("{} records created", counter);
        HttpResponse::Created().body(res)
    } else {
        HttpResponse::InternalServerError().finish()
    }

}