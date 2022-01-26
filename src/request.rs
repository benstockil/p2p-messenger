use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

use crate::objects::{Message, Peer};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    NewClient(Peer),
    Message(Message),
    Debug(String),
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Ok,
    Err,
}

pub type ReqRecvPair = (Request, Sender<Response>);
