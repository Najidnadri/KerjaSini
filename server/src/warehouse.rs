use std::collections::HashMap;

use actix::{Actor, Context, Handler, Addr};
use uuid::Uuid;

use crate::{messages::{Disconnect, Connect, ClientMessage, WsMessage}, ws::WsConn};



pub struct Warehouse {
    active: HashMap<Uuid, Addr<WsConn>> //<id, temp_key>
}

impl Default for Warehouse {
    fn default() -> Self {
        Warehouse { active: HashMap::new() }
    }
}

impl Actor for Warehouse {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Warehouse {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        if self.active.remove(&msg.id).is_some() {
           println!("connection: {} disconnected", &msg.id);
        } 
    }
}

impl Handler<Connect> for Warehouse {
    type Result = ();
    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        self.active.insert(msg.id.clone(), msg.addr);
        println!("Connection: {}, connected", msg.id);
    }
}

impl Handler<ClientMessage> for Warehouse {
    type Result = String;
    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("handle things here");
        let client_addr = self.active.get(&msg.id).unwrap();
        client_addr.do_send(WsMessage {text: msg.msg});
        "Returning message from Warehouse ClientMessage Handler!".to_string()
    }
}