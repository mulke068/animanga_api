use actix_web::{web, HttpResponse, Responder};

use crate::AppData;

// ---------------------------- Imports ------------------------------
// ---------------------------- Handlers ------------------------------
pub async fn get(state: web::Data<AppData>) -> impl Responder {
    let mut res = String::from("");

    if let Ok(_surreal) = state.surreal.health().await {
        res.push_str("Surreal is connected");
    } else {
        res.push_str("Surreal is not connected");
    }

    if let Ok(_redis) = state.redis.get_connection() {
        res.push_str("\nRedis is connected");
    } else {
        res.push_str("\nRedis is not connected");
    }

    // res.push_str(&format!("{:#?}", surreal));
    // res.push_str(&format!("{:#?}", redis));
    // res.push_str(&format!("{:#?}", meili));
    HttpResponse::Ok().body(res)
    // HttpResponse::BadGateway()
}
