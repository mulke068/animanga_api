// ---------------------------- Imports ------------------------------
use actix_web::{web, HttpResponse, Responder};

use crate::AppServices;

// ---------------------------- services ------------------------------
// ---------------------------- handlers ------------------------------
pub async fn post(service: web::Data<AppServices>) -> impl Responder {
    let import = service.surreal.import("backup.sql").await.unwrap();
    let res = format!("{:#?}", import);
    HttpResponse::Ok().body(res)
}
