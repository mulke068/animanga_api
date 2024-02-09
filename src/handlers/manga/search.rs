use std::collections::HashMap;

// ---------------------- Imports -------------------
use actix_web::{
    web::{self, Query},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::AppServices;

use super::main::MangaNames;

// ---------------------- Structs -------------------

#[derive(Debug, Deserialize, Serialize)]
pub struct MangaSearch {
    id: Thing,
    names: MangaNames,
}

#[derive(Deserialize)]
struct SearchForm {
    q: String,
    l: usize,
}

// ---------------------- Handlers -------------------
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

    let record: Vec<MangaSearch> = service
        .surreal
        .query("SELECT id,names FROM manga;")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    if record.len() != 0 {
        let mut filtered_data: Vec<&MangaSearch> = record
            .iter()
            .filter(|res| {
                res.names.original.contains(&param.q)
                    || res.names.en.contains(&param.q)
                    || res.names.jp.contains(&param.q)
            })
            .collect();

        if param.l != 0 {
            let lenght = param.l.min(filtered_data.len());
            filtered_data.truncate(lenght);
        }

        let res = serde_json::to_string(&filtered_data).unwrap_or("No Data Found".to_string());

        HttpResponse::Ok().body(res)
    } else {
        HttpResponse::NotFound().body("Not Found")
    }
}
