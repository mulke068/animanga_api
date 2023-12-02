#![recursion_limit = "500"]

// -------------------- Imports --------------------
use actix_web::{/*guard,*/ middleware::Logger, web, App, HttpResponse, HttpServer, Responder,};
use log::{error, info};
use meilisearch_sdk::settings;
// ------------------------------------------------
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
// --------------------- Main ---------------------
use once_cell::sync::Lazy;
#[allow(non_upper_case_globals)]
static client_surreal: Lazy<Surreal<surrealdb::engine::remote::ws::Client>> =
    Lazy::new(|| Surreal::init());

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "debug");
    // std::env::set_var("RUST_BACKTRACE", "0");
    dotenv::dotenv().ok();

    let surreal_url =
        std::env::var("SURREAL_URL").unwrap_or_else(|_| String::from("127.0.0.1:8008"));
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| String::from("redis://127.0.0.1"));
    let meilisearch_url =
        std::env::var("MEILISEARCH_URL").unwrap_or_else(|_| String::from("http://127.0.0.1:7700"));

    api::middleware::logger::setup_logger();

    match client_surreal.connect::<Ws>(surreal_url).await {
        Ok(client) => {
            info!("Connected to surrealdb");
            client
        }
        Err(_) => error!("Failed to connect to surrealdb"),
    };

    match client_surreal
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
    {
        Ok(signin) => Some(signin),
        Err(_) => {
            error!("Failed to signin");
            None
        }
    };
    match client_surreal.use_ns("test").use_db("test").await {
        Ok(db) => db,
        Err(_) => error!("Failed to connect to Namespace and Database"),
    }

    // let client_surreal: Surreal<surrealdb::engine::remote::ws::Client> =
    //     match Surreal::new::<Ws>(surreal_url).await {
    //         Ok(client) => {
    //             info!("Connected to surrealdb");
    //             client
    //                 .signin(Root {
    //                     username: "root",
    //                     password: "root",
    //                 })
    //                 .await
    //                 .unwrap();
    //             client.use_ns("test").use_db("test").await.unwrap();
    //             client
    //         }
    //         Err(_) => {
    //             error!("Failed to connect to surrealdb");
    //             return Ok(());
    //         }
    //     };

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

    let client_meilisearch: meilisearch_sdk::Client =
        meilisearch_sdk::Client::new(meilisearch_url, Some("MASTER_KEY"));

    {
        let settings = settings::Settings {
            ranking_rules: Some(vec![
                "typo".to_string(),
                "words".to_string(),
                "proximity".to_string(),
                "attribute".to_string(),
                "wordsPosition".to_string(),
                "exactness".to_string(),
                "release_date:desc".to_string(),
                "score:desc".to_string(),
            ]),
            ..Default::default()
        };
        client_meilisearch.index("anime").set_settings(&settings).await.unwrap();
        client_meilisearch.index("manga").set_settings(&settings).await.unwrap();
    }

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(
                "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .app_data(web::Data::new(api::AppData {
                surreal: client_surreal.clone(),
                redis: client_redis.clone(),
                meilisearch: client_meilisearch.clone(),
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
                web::resource("/user")
                    .name("user")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::user::main::handler_user_get))
                    .route(web::post().to(api::handlers::user::main::handler_user_post))
                    .route(web::patch().to(api::handlers::user::main::handler_user_patch))
                    .route(web::delete().to(api::handlers::user::main::handler_user_delete)),
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
                web::resource("/manga")
                    .name("manga")
                    // .guard(guard::Header("Content-Type", "application/json"))
                    .route(web::get().to(api::handlers::manga::main::handler_manga_get))
                    .route(web::post().to(api::handlers::manga::main::handler_manga_post))
                    .route(web::patch().to(api::handlers::manga::main::handler_manga_patch))
                    .route(web::delete().to(api::handlers::manga::main::handler_manga_delete)),
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
            .service(
                web::resource("/anime/search")
                    .route(web::get().to(api::handlers::anime::search::get))
                    .route(web::post().to(api::handlers::anime::search::post)),
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
                    .route(web::get().to(api::handlers::manga::search::get))
                    .route(web::post().to(api::handlers::manga::search::post)),
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// ------------------------------------------------
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Home Page\n-> to /user\n-> to /anime\n-> to /manga")
}
