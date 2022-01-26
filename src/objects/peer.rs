use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub address: IpAddr,
}
