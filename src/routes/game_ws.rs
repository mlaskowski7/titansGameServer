use actix::{Actor, ActorContext, StreamHandler};
use actix_web::rt::time::Instant;
use serde::{Deserialize, Serialize};
use actix_web_actors::ws;

#[derive(Serialize, Deserialize)]
struct Player {
    user_id: String,
    position: (i32, i32),
}

struct WebSocket {
    hb: Instant,
}

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {

            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => ctx.stop(),
        }
    }
}