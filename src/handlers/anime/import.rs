use actix_web::{web, HttpResponse, Responder};
use surrealdb::sql::Datetime;

use crate::AppServices;

use super::main::*;

pub async fn post(req: web::Json<Vec<Anime>>, services: web::Data<AppServices>) -> impl Responder {
    let mut counter: usize = 0;
    let mut error_counter: usize = 0;

    for anime in req.iter() {
        let _record: Vec<AnimeRecord> = match services.surreal.create("anime").content(AnimeCreate {
            base: anime.base(),
            updated_at: Datetime::default(),
            created_at: Datetime::default(),
        }).await {
            Ok(data) => data,
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