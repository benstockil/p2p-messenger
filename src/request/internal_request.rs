use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

use crate::objects::{Message, Peer};

#[derive(Debug, Serialize, Deserialize)]
pub enum InternalRequest {
    NewClient(Peer),
    Message(Message),
    Debug(String),
    Stop,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InternalResponse {
    Ok,
    Err,
}

pub type IRRPair = (InternalRequest, Sender<InternalResponse>);
