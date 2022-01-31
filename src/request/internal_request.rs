use serde::{Deserialize, Serialize};
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

