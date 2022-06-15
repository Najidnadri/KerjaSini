use std::time::{Duration, Instant};
use actix::{Addr, Actor, AsyncContext, WrapFuture, fut, ActorContext, StreamHandler, Handler, ActorFutureExt, ContextFutureSpawner};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::{messages::{Disconnect, Connect, ClientMessage, WsMessage}, warehouse::Warehouse};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Clone)]
pub enum Side {
    Employer,
    Employee
}

pub struct WsConn {
    id: Uuid,
    temp_key: Uuid,
    warehouse_addr: Addr<Warehouse>,
    hb: Instant,
    side: Side
}

impl WsConn{
    pub fn new(temp_key: Uuid, warehouse_addr: Addr<Warehouse>, side: Side) -> Self {
        WsConn {
            id: Uuid::new_v4(),
            temp_key,
            warehouse_addr,
            hb: Instant::now(),
            side
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.warehouse_addr.send(Connect {
            addr,
            id: self.id,
            temp_key: self.temp_key
        })
        .into_actor(self)
        .then(|res, _, ctx| {
            match res {
                Ok(_) => (),
                _ => ctx.stop()
            }
            fut::ready(())
        })
        .wait(ctx)
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.warehouse_addr.do_send(Disconnect {
            id: self.id
        });
        actix::Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMOUT {
                println!("disconnected because heartbeat error");
                act.warehouse_addr.do_send(Disconnect {
                    id: act.id
                });
                ctx.stop();
            }
            ctx.ping(b"PING");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
                todo!();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(msg)) => {
                println!("sending ClientMessage to warehouse");
                let a = self.warehouse_addr.do_send(ClientMessage {
                    id: self.id,
                    temp_key: self.temp_key,
                    side: self.side.clone(),
                    msg: msg.to_string(),
                });
            }
            Err(_e) => {
                panic!();
            },
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("Handler in WsMessage");
        ctx.text(msg.text);
    }
}