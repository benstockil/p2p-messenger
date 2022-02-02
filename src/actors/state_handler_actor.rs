use async_trait::async_trait;
use tokio::io;
use tokio::sync::{mpsc, oneshot};
use crate::actors::{PeerListener, ClientHandler};
use crate::objects::{Message, Peer};
use crate::request::InternalResponse;
use super::actor::Actor;

type RRPair = (
    <StateHandlerActor as Actor>::Request,
    oneshot::Sender<<StateHandlerActor as Actor>::Response>,
);

pub struct StateHandlerConfig {}

#[derive(Debug)]
pub struct StateHandlerActor {
    peers: Vec<ClientHandler>,
    listener: Option<PeerListener>,
    rx: mpsc::UnboundedReceiver<RRPair>,
}

#[async_trait]
impl Actor for StateHandlerActor {
    type Request = Request;
    type Response = InternalResponse;
    type Config = StateHandlerConfig;

    fn new(rx: mpsc::UnboundedReceiver<RRPair>, config: Self::Config) -> Self {
        Self {
            rx,
            peers: Vec::new(),
            listener: None,
        }
    }

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

#[derive(Debug)]
pub enum Request {
    NewClient(ClientHandler),
    Message(Message),
    Debug(String),
}
