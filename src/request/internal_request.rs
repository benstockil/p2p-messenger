use serde::{Deserialize, Serialize};
use crate::objects::{Message, Peer};

pub trait Request {
    const STOP_SIGNAL: Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InternalRequest {
    NewClient(Peer),
    Message(Message),
    Debug(String),
    Stop,
}

impl Request for InternalRequest {
    const STOP_SIGNAL: Self = Self::Stop;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum InternalResponse {
    Ok,
    Err,
}

