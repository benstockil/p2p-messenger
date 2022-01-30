use async_trait::async_trait;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use crate::request::{InternalRequest, InternalResponse};
use crate::actors::{ClientHandler, StateHandler};
use super::actor::Actor;

type RRPair = (
    <PeerListenerActor as Actor>::Request,
    oneshot::Sender<<PeerListenerActor as Actor>::Response>,
);

pub struct PeerListenerConfig {
    pub state_handler: StateHandler,
}

pub struct PeerListenerActor {
    state_handler: StateHandler,
    rx: mpsc::UnboundedReceiver<RRPair>,
}

#[async_trait]
impl Actor for PeerListenerActor {
    type Request = InternalRequest;
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
            // self.state_handler.send(
            //     InternalRequest::NewClient(Peer { address: "192.168.0.43".parse().unwrap() })
            // );
        }
    }
}
