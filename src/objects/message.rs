use serde::{Serialize, Deserialize};
use super::{GroupId, UserId};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub author: UserId,
    pub group: GroupId,
}
