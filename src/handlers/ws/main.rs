
use actix::{Actor, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}


impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => (),
            Ok(ws::Message::Text(text)) => {
                log::info!(target: "ws::Message::Text", "text: {:?}", text);
                if text == "ping" || text == "Ping" {
                    ctx.text("Pong")
                } else {
                    ctx.text(text)
                }
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
            }
            _ => ()
        }
    }
}

pub async fn handler_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let res = ws::start(MyWs, &req, stream);
    log::info!(target: "ws::start" , "result: {:?}", res);
    res
} 