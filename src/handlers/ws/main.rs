use crate::handlers::anime::model::AnimeUpdate;
use crate::handlers::anime::model::{Anime, AnimeCreate, AnimeRecord};
use actix::{Actor, StreamHandler};
use actix_web::{Error, HttpRequest, HttpResponse, Result, web};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Datetime;
// use crate::handlers::manga::model::Manga;
// use crate::handlers::manga::model::User;

use crate::AppServices;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "target", content = "payload")]
pub enum WsPayload {
    #[serde(rename = "anime")]
    Anime(AnimeMessage),
    #[serde(rename = "manga")]
    Manga(MangaMessage),
    #[serde(rename = "user")]
    User(UserMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum AnimeMessage {
    #[serde(rename = "create")]
    Create {
        data: Anime,
        request_id: Option<String>,
    },

    #[serde(rename = "update")]
    Update {
        id: String,
        data: Anime,
        request_id: Option<String>,
    },

    #[serde(rename = "get")]
    Get {
        id: String,
        request_id: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum MangaMessage {
    #[serde(rename = "noop")]
    NoOp,
    // #[serde(rename = "create")]
    // Create { data: Manga, request_id: Option<String> },

    // #[serde(rename = "update")]
    // Update { id: String, data: Manga, request_id: Option<String> },

    // #[serde(rename = "get")]
    // Get { id: String, request_id: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
pub enum UserMessage {
    #[serde(rename = "noop")]
    NoOp,
    // #[serde(rename = "create")]
    // Create { data: User, request_id: Option<String> },

    // #[serde(rename = "update")]
    // Update { id: String, data: User, request_id: Option<String> },

    // #[serde(rename = "get")]
    // Get { id: String, request_id: Option<String> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyWs {
    pub services: AppServices,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(msg)) => {
                // log::info!(target: "ws::Message::Text", "text: {:?}", &msg);

                match serde_json::from_str::<WsPayload>(&msg) {
                    Ok(ws_msg) => match ws_msg {
                        WsPayload::Anime(anime_msg) => handle_anime(&self.services, anime_msg, ctx),
                        WsPayload::Manga(manga_msg) => handle_manga(&self.services, manga_msg, ctx),
                        WsPayload::User(user_msg) => handle_user(&self.services, user_msg, ctx),
                    },
                    Err(err) => {
                        log::error!(target: "ws::Message::Text", "Failed to parse message: {:?}", err);
                        ctx.text(
                            json!({
                                "target": "error",
                                "error": "Invalid WS payload",
                                "details": err.to_string()
                            })
                            .to_string(),
                        );
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => (),
        }
    }
}

pub async fn handler_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let buff_service = req.app_data::<web::Data<AppServices>>().unwrap().as_ref();
    let res = ws::start(
        MyWs {
            services: buff_service.clone(),
        },
        &req,
        stream,
    );
    // log::info!(target: "ws::start" , "result: {:?}", res);
    res
}


use actix_web_actors::ws::WebsocketContext;
use actix::prelude::*;
use actix::fut;

fn handle_anime(services: &AppServices, msg: AnimeMessage, ctx: &mut WebsocketContext<MyWs>) {
    match msg {
        AnimeMessage::Create { data, request_id } => {
            let db = services.surreal.clone();
            let create = AnimeCreate {
                base: data,
                updated_at: Datetime::default(),
                created_at: Datetime::default(),
            };

            let fut = async move {
                match db.create::<Option<AnimeRecord>>("anime").content(create).await {
                    Ok(Some(rec)) => json!({
                        "target": "anime",
                        "action": "created",
                        "request_id": request_id,
                        "data": rec,
                    }),
                    Ok(None) => json!({
                        "target": "error",
                        "error": "No record created",
                        "request_id": request_id
                    }),
                    Err(err) => {
                        log::error!(target: "ws::handle_anime", "Error creating anime: {:?}", err);
                        json!({
                            "target": "error",
                            "error": "Failed to create anime",
                            "details": err.to_string(),
                            "request_id": request_id
                        })
                    }
                }
            };

            ctx.spawn(
                fut::wrap_future(fut)
                    .map(|res, _act: &mut MyWs, ctx: &mut WebsocketContext<MyWs>| {
                        ctx.text(res.to_string());
                    })
            );
        }
        AnimeMessage::Update { id, data, request_id } => {
            let db = services.surreal.clone();
            let update = AnimeUpdate {
                base: data,
                updated_at: Datetime::default()
            };

            let fut = async move {
                match db.update::<Option<AnimeRecord>>(("anime", id)).merge(update).await {
                    Ok(Some(rec)) => json!({
                        "target": "anime",
                        "action": "updated",
                        "request_id": request_id,
                        "data": rec,
                    }),
                    Ok(None) => json!({
                        "target": "error",
                        "error": "No record updated",
                        "request_id": request_id
                    }),
                    Err(err) => {
                        log::error!(target: "ws::handle_anime", "Error updating anime: {:?}", err);
                        json!({
                            "target": "error",
                            "error": "Failed to update anime",
                            "details": err.to_string(),
                            "request_id": request_id
                        })
                    }
                }
            };

            ctx.spawn(
                fut::wrap_future(fut).map(
                    |res, _act: &mut MyWs, ctx: &mut WebsocketContext<MyWs>| {
                        ctx.text(res.to_string());
                    }
                )
            );
        }
        AnimeMessage::Get { id, request_id } => {
            let db = services.surreal.clone();

            let fut = async move {
                match db.select::<Option<AnimeRecord>>(("anime", id)).await {
                    Ok(Some(rec)) => json!({
                        "target": "anime",
                        "action": "got",
                        "request_id": request_id,
                        "data": rec,
                    }),
                    Ok(None) => json!({
                        "target": "error",
                        "error": "No record found",
                        "request_id": request_id
                    }),
                    Err(err) => {
                        log::error!(target: "ws::handle_anime", "Error getting anime: {:?}", err);
                        json!({
                            "target": "error",
                            "error": "Failed to get anime",
                            "details": err.to_string(),
                            "request_id": request_id
                        })
                    }
                }
            };

            ctx.spawn(fut::wrap_future(fut).map(
                |res, _act: &mut MyWs, ctx: &mut WebsocketContext<MyWs>| {
                    ctx.text(res.to_string());
                }
            ));
            // ctx.text(json!({
            //     "target": "anime",
            //     "payload": {
            //         "action": "got",
            //         "request_id": request_id,
            //         "id": id,
            //         "data": {
            //             "mock": true
            //         }
            //     }
            // }).to_string());
        }
    }
}

fn handle_manga(_services: &AppServices, _msg: MangaMessage, _ctx: &mut WebsocketContext<MyWs>) {
    return;
}
fn handle_user(_services: &AppServices, _msg: UserMessage, _ctx: &mut WebsocketContext<MyWs>) {
    return;
}