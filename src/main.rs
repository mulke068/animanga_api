#![recursion_limit = "500"]

use std::time::Duration;

// -------------------- Imports --------------------
use actix_cors::Cors;
use actix_web::{/*guard,*/ middleware::Logger, web, App, HttpResponse, HttpServer, Responder,};
use log::{error, info};
// ------------------------------------------------
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
// --------------------- Main ---------------------
use once_cell::sync::Lazy;
use tokio::time::sleep;

#[allow(non_upper_case_globals)]
static client_surreal: Lazy<Surreal<surrealdb::engine::remote::ws::Client>> =
    Lazy::new(|| Surreal::init());

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "0");
    dotenv::dotenv().ok();

    let surreal_url =
        std::env::var("SURREAL_URL").unwrap_or_else(|_| String::from("127.0.0.1:8000"));
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
    let app_url: String = std::env::var("API_URL").unwrap_or_else(|_| String::from("172.0.0.1"));
    let app_port: String = std::env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());

    api::middleware::logger::setup_logger();

    {
        info!("Starting Server");
        info!("Surreal URL          : {}", &surreal_url);
        info!("Redis URL            : {}", &redis_url);
        info!("APP v4 Running on    : {}:{}", &app_url, &app_port);
        // info!("APP v6 Running on: [::1]:{}", &app_port);
    }


    loop {
        match client_surreal.connect::<Ws>(surreal_url.clone()).await {
            Ok(_) => {
                info!("Connected to SurrealDB");

                match client_surreal
                    .signin(Root {
                        username: "root",
                        password: "root",
                    })
                    .await
                {
                    Ok(_) => {
                        info!("Signed into SurrealDB");

                        match client_surreal.use_ns("test").use_db("test").await {
                            Ok(_) => {
                                info!("Connected to SurrealDB namespace and database");
                                break;
                            }
                            Err(_) => {
                                error!("Failed to connect to SurrealDB namespace and database: . Retrying in 1 second...");
                            }
                        }
                    }
                    Err(_) => {
                        error!("Failed to signin to SurrealDB: . Retrying in 1 second...");
                    }
                }
            }
            Err(_) => {
                error!("Failed to connect to SurrealDB: . Retrying in 1 second...");
            }
        }

        sleep(Duration::from_secs(1)).await;
        log::info!("Retrying to connect to SurrealDB");
    }

    let client_redis: redis::Client = match redis::Client::open(redis_url) {
        Ok(client) => {
            info!("Open Connection to redis");
            {
                match client.get_connection() {
                    Ok(_) => info!("Connected to redis"),
                    Err(_) => error!("Failed to connect to redis"),
                }
            }
            client
        }
        Err(_) => {
            error!("Failed to open Connection to redis");
            return Ok(());
        }
    };

    //{
    //    let settings = settings::Settings {
    //      ranking_rules: Some(vec![
    //            "typo".to_string(),
    //            "words".to_string(),
    //            "proximity".to_string(),
    //            "attribute".to_string(),
    //            "wordsPosition".to_string(),
    //            "exactness".to_string(),
    //            "release_date:desc".to_string(),
    //            "score:desc".to_string(),
    //        ]),
    //        ..Default::default()
    //    };

    HttpServer::new(move || {
        let logger = Logger::new("%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T");

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(logger)
            .wrap(cors)
            .app_data(web::Data::new(api::AppServices {
                surreal: client_surreal.clone(),
                redis: client_redis.clone(),
            }))
            // .wrap_fn(api::middleware::cache_middleware)
            // .wrap_fn(|req, srv| {
            //     let future = srv.call(req);
            //     // debug!("{:#?}", req);
            //     async {
            //         let res = future.await.unwrap();
            //         let test = res.response_mut().body();
            //         debug!("{:#?}", test.borrow_mut());
            //         Ok(res)
            //     }
            // })
            .service(
                web::resource("/ws").route(web::get().to(api::handlers::ws::main::handler_ws)),
            )
            .service(
                web::resource("/user")
                    .name("user")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::user::main::handler_user_get))
                    .route(web::post().to(api::handlers::user::main::handler_user_post))
                    .route(web::patch().to(api::handlers::user::main::handler_user_patch))
                    .route(web::delete().to(api::handlers::user::main::handler_user_delete)),
            )
            .service(
                web::resource("/user/auth")
                    .name("user_auth")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::user::auth::handler_user_auth_get))
                    .route(web::post().to(api::handlers::user::auth::handler_user_login_post)),
            )
            .service(
                web::resource("/anime")
                    .name("anime")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::anime::main::handler_anime_get))
                    .route(web::post().to(api::handlers::anime::main::handler_anime_post))
                    .route(web::patch().to(api::handlers::anime::main::handler_anime_patch))
                    .route(web::delete().to(api::handlers::anime::main::handler_anime_delete)),
            )
            .service(
                web::resource("/anime/user")
                    .name("user_anime")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::anime::user::handler_user_anime_get))
                    .route(web::post().to(api::handlers::anime::user::handler_user_anime_post))
                    .route(web::patch().to(api::handlers::anime::user::handler_user_anime_patch))
                    .route(web::delete().to(api::handlers::anime::user::handler_user_anime_delete)),
            )
            .route(
                "/anime/import",
                web::post().to(api::handlers::anime::main::handler_anime_multi_post),
            )
            .service(
                web::resource("/anime/search")
                    .route(web::get().to(api::handlers::anime::search::get))
                    .route(web::post().to(api::handlers::anime::search::post)),
            )
            .service(
                web::resource("/manga")
                    .name("manga")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::manga::main::handler_manga_get))
                    .route(web::post().to(api::handlers::manga::main::handler_manga_post))
                    .route(web::patch().to(api::handlers::manga::main::handler_manga_patch))
                    .route(web::delete().to(api::handlers::manga::main::handler_manga_delete)),
            )
            .service(
                web::resource("/manga/user")
                    .name("user_manga")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::manga::user::handler_user_manga_get))
                    .route(web::post().to(api::handlers::manga::user::handler_user_manga_post))
                    .route(web::patch().to(api::handlers::manga::user::handler_user_manga_patch))
                    .route(web::delete().to(api::handlers::manga::user::handler_user_manga_delete)),
            )
            .service(
                web::resource("/manga/search")
                    .route(web::get().to(api::handlers::manga::search::get)), // .route(web::post().to(api::handlers::manga::search::post)),
            )
            .service(
                web::scope("/status")
                    .route("/", web::get().to(api::handlers::status::main::get))
                    .route(
                        "/import",
                        web::post().to(api::handlers::status::import::post),
                    )
                    .route("/export", web::get().to(api::handlers::status::export::get)),
            )
            .route("/", web::get().to(main_page))
    })
    .workers(1)
    .bind((
        app_url.as_str(),
        app_port.to_string().parse::<u16>().unwrap(),
    ))?
    // .bind(format!("[[::1]]:{app_port}"))?
    .run()
    .await
}

// ------------------------------------------------
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Home Page\n-> to /user\n-> to /anime\n-> to /manga\n-> to /status\n-> to /ws")
}
