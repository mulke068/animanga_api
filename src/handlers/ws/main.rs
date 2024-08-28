use actix::{ Actor, StreamHandler };
use actix_web::{ web, Error, HttpRequest, HttpResponse, Result };
use actix_web_actors::ws;
use serde::{ Deserialize, Serialize };

use crate::AppServices;

#[derive(Debug, Deserialize, Serialize)]
struct PayloadHandler {
    token: String,
    id: String,
    value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SendMessageHandler {
    target: String,
    payload: PayloadHandler,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecieveMessageHandler {
    target: String,
    payload: PayloadHandler,
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

                let deserialized: RecieveMessageHandler = serde_json::from_str(&msg).unwrap();
                let target: Vec<&str> = deserialized.target.split(':').collect();

                match target[0] {
                    "has_anime" => {
                        match target[1] {
                            "watched" => {
                                

                                ctx.text("Test 1");
                            }
                            "count" => {
                                ctx.text("Test 2");
                            }
                            _ => {
                                ctx.text("Error");
                            }
                        }
                    }
                    _ => { ctx.text(msg) }
                }

                // log::info!("text: {:?}", slplitted);
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
    let res = ws::start(MyWs { services: buff_service.clone() }, &req, stream);
    // log::info!(target: "ws::start" , "result: {:?}", res);
    res
}
