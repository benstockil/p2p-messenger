use serde::{Serialize, Deserialize};
use super::user::UserId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub author: UserId,
}
