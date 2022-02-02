use async_trait::async_trait;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use crate::objects::Peer;
use crate::request::InternalResponse;
use crate::actors::{ClientHandler, StateHandler};
use super::actor::Actor;

type RRPair = (
    <PeerListenerActor as Actor>::Request,
    oneshot::Sender<<PeerListenerActor as Actor>::Response>,
);

pub struct PeerListenerConfig {
    pub state_handler: StateHandler,
}

#[derive(Debug)]
pub struct PeerListenerActor {
    state_handler: StateHandler,
    rx: mpsc::UnboundedReceiver<RRPair>,
}

#[async_trait]
impl Actor for PeerListenerActor {
    type Request = Request;
    type Response = InternalResponse;
    type Config = PeerListenerConfig;

    fn new(rx: mpsc::UnboundedReceiver<RRPair>, config: Self::Config) -> Self {
        Self {
            rx,
            state_handler: config.state_handler,
        }
    }

    async fn run(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3030").await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let client_handler = ClientHandler::new(socket, self.state_handler.clone());
            self.state_handler.new_client(client_handler).await;
        }
    }
}

#[derive(Debug)]
pub enum Request {}
