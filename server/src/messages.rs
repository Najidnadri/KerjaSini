use actix::{Message, Addr};
use uuid::Uuid;

use crate::ws::{Side, WsConn};



#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub text: String,
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Addr<WsConn>,
    pub id: Uuid,
    pub temp_key: Uuid
}

#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid
}

#[derive(Message, Debug)]
#[rtype(result = "String")]
pub struct ClientMessage {
    pub id: Uuid,
    pub temp_key: Uuid,
    pub side: Side,
    pub msg: String
}