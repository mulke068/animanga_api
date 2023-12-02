// ---------------------------- Imports ------------------------------
use actix_web::{web, HttpResponse, Responder};

use crate::AppData;

// ---------------------------- States ------------------------------
// ---------------------------- handlers ------------------------------
pub async fn post(state: web::Data<AppData>) -> impl Responder {
    let import = state.surreal.import("backup.sql").await.unwrap();
    let res = format!("{:#?}", import);
    HttpResponse::Ok().body(res)
}
