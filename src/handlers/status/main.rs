use actix_web::{web, HttpResponse, Responder};

use crate::AppData;

// ---------------------------- Imports ------------------------------
// ---------------------------- Handlers ------------------------------
pub async fn get(_state: web::Data<AppData>) -> impl Responder {
    // let surreal = state.surreal.health().await.unwrap();
    // let meili = state.meilisearch.health().await.unwrap();
    // // let redis = state.redis.get_connection().unwrap();
    // let mut res = String::from("");
    // res.push_str(&format!("{:#?}", surreal));
    // res.push_str(&format!("{:#?}", meili));
    // HttpResponse::Ok().body(res)
    HttpResponse::BadGateway()
}
