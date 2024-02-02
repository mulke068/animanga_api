use actix_web::{web, HttpResponse, Responder};

use crate::AppData;

// ---------------------------- Imports ------------------------------
// ---------------------------- Handlers ------------------------------
pub async fn get(state: web::Data<AppData>) -> impl Responder {
    let surreal_export = state.surreal.export("backup.sql").await.unwrap();
    // let meili_export = state.meilisearch.list_all_indexes().await.unwrap();
    let res = format!("{:#?}", surreal_export);
    // let res = format!("{:#?}", meili_export);
    HttpResponse::Ok().body(res)
}
