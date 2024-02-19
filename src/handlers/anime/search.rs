use std::collections::HashMap;

use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::main::AnimeNames;
use crate::AppServices;

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeSearch {
    id: Thing,
    names: AnimeNames,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchForm {
    q: String,
    l: usize,
}

pub async fn get(params: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let query_string = params.query_string();
    let mut q = "".to_string();
    let mut l = 0;

    for (key, value) in Query::<HashMap<String, String>>::from_query(query_string)
        .unwrap_or_else(|_| Query(HashMap::new()))
        .into_inner()
    {
        match key.as_str() {
            "q" => q = value,
            "l" => l = value.parse().unwrap_or(0),
            _ => (),
        }
    }

    let param = SearchForm { q, l };

    let record: Vec<AnimeSearch> = service
        .surreal
        .query("SELECT id,names FROM anime")
        .await
        .unwrap()
        .take(0)
        .unwrap_or(vec![]);

    if record.len() != 0 {
        let mut filtered_data: Vec<&AnimeSearch> = record
            .iter()
            .filter(|res| {
                res.names.original.contains(&param.q)
                    || res.names.en.as_ref().map(|en| en.contains(&param.q)).unwrap_or(false)
                    || res.names.jp.as_ref().map(|jp| jp.contains(&param.q)).unwrap_or(false)
            })
            .collect();

        if param.l != 0 {
            // Ensure param.l is within the bounds of filtered_data
            let length = param.l.min(filtered_data.len());
            filtered_data.truncate(length);
        }

        let res = serde_json::to_string(&filtered_data)
            .unwrap_or_else(|_| panic!("Failed to format the data"));

        return HttpResponse::Ok().body(res);
    } else {
        return HttpResponse::NotFound().body("No Data Found");
    }
}

pub async fn post(req: HttpRequest, service: web::Data<AppServices>) -> impl Responder {
    let query_string = req.query_string();
    let mut q = "".to_string();
    let mut l = 0;

    for (key, value) in Query::<HashMap<String, String>>::from_query(query_string).unwrap_or_else(|_| Query(HashMap::new())).into_inner() {
        match key.as_str() {
            "q" => q = value,
            "l" => l = value.parse().unwrap_or(0),
            _ => (),
        }
    }

    let param = SearchForm { q, l };

    // let cache = Caching::new(req.uri().to_string(), &service);

    // match cache.get().await {
    //     Some(data) => {
    //         HttpResponse::Ok().append_header(("X-Cache-Remaining-Time", cache.timer_get().await.to_string())).body(data)
    //     }
    //     None => {
    //         let res = super::request::module_request_jikan(param.q).await.unwrap();
    //         {
    //             cache.set(&res).await;
    //         }
    //         HttpResponse::Ok().body(res)
    //     }
    // }
    let res = super::request::module_request_jikan(param.q, service).await.unwrap();

    HttpResponse::Ok().body(res)
}