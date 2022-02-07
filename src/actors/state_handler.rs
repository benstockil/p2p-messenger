use async_trait::async_trait;
use tokio::io;
use tokio::sync::{mpsc, oneshot};
use crate::actors::{PeerListener, ClientHandler};
use crate::objects::{Message, Peer};
use crate::request::InternalResponse;
use crate::actor::{Actor, Address, Envelope};

#[derive(Debug)]
pub struct StateHandler {
    peers: Vec<Address<ClientHandler>>,
    listener: Address<PeerListener>,
    rx: mpsc::UnboundedReceiver<Envelope>,
}

#[async_trait]
impl Actor for StateHandler {
    async fn run(&mut self) -> io::Result<()> {
        let mut run = true;
        while run {
            let (incoming, response_channel) = self.rx.recv().await.unwrap();
            match incoming {
                // Request::Stop => { run = false; },
                Request::NewClient(client_handler) => {
                    self.peers.push(client_handler);
                }
                _ => { dbg!(incoming); }
            }
            response_channel.send(InternalResponse::Ok).unwrap();
        }
        Ok(())
    }
}

impl StateHandler {
    fn new(rx: mpsc::UnboundedReceiver<Envelope>) -> Self {
        Self {
            rx,
            peers: Vec::new(),
            listener: None,
        }
    }
}
pub enum Request {
    NewClient(ClientHandler),
    Message(Message),
    Debug(String),
}
