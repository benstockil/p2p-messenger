use serde::{Serialize, Deserialize};
use crate::objects::Message;

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteRequest {
    Message(Message),
}
