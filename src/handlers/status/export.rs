use actix_web::{web, HttpResponse, Responder};

use crate::AppServices;

// ---------------------------- Imports ------------------------------
// ---------------------------- Handlers ------------------------------
pub async fn get(service: web::Data<AppServices>) -> impl Responder {
    let surreal_export = service.surreal.export("backup.sql").await.unwrap();
    // let meili_export = service.meilisearch.list_all_indexes().await.unwrap();
    let res = format!("{:#?}", surreal_export);
    // let res = format!("{:#?}", meili_export);
    HttpResponse::Ok().body(res)
}
