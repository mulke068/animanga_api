use std::collections::HashMap;

use actix_web::{
    web::{self},
    HttpRequest, HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::main::AnimeNames;
use crate::AppData;

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

pub async fn get(params: HttpRequest, state: web::Data<AppData>) -> impl Responder {
    let query_string = params.query_string();
    let mut q = "".to_string();
    let mut l = 0;

    for (key, value) in web::Query::<HashMap<String, String>>::from_query(query_string)
        .unwrap_or_else(|_| actix_web::web::Query(HashMap::new()))
        .into_inner()
    {
        match key.as_str() {
            "q" => q = value,
            "l" => l = value.parse().unwrap_or(0),
            _ => (),
        }
    }

    let param = SearchForm { q, l };

    let record: Vec<AnimeSearch> = state
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
                    || res.names.en.contains(&param.q)
                    || res.names.jp.contains(&param.q)
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
