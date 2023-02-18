use crate::lobby::Lobby;
use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::{fut, ActorContext};
use actix::{Actor, ActorFuture, Addr, ContextFutureSpawner, Running, StreamHandler, WrapFuture};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

const HEARTBEAT_INTERVAl: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct WsConn {
    room: Uuid,
    lobby_actor: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
}

impl WsConn {
    pub fn new(room: Uuid, lobby: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            room,
            hb: Instant::now(),
            lobby_addr: lobby,
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAl, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting... failed heartbeat");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room,
                });
                ctx.stop();
                return;
            }

            ctx.ping(b"PING")
        });
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room,
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room,
        });
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Client heartbeating us
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg)
            }
            // Response to ping, reset our clock, connection alive
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // If the message is binary then we send it to the context
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            // Close
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            // TODO: Implement continuation frames (WebSocket messages that could not fit
            // into one message)
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            // No operation
            Ok(ws::Message::Nop) => (),
            // On text message we send it to the lobby for brokering where it needs to go
            Ok(Text(s)) => self.looby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s,
                room_id: self.room,
            }),
            // TODO: Handle error more gracefully
            Err(e) => panic!(e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
