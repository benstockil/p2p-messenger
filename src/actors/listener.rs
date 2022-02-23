use async_trait::async_trait;
use tokio::io;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, oneshot};
use crate::objects::Peer;
use crate::actors::{ClientHandler, StateHandler};
use crate::actor::{Actor, Address, Envelope};

#[derive(Debug)]
pub struct PeerListener {
    state_handler: Address<StateHandler>,
    rx: mpsc::UnboundedReceiver<Envelope>,
}

#[async_trait]
impl Actor for PeerListener {
    async fn run(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3030").await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let client_handler = ClientHandler::new(socket, self.state_handler.clone());
            self.state_handler.new_client(client_handler).await;
        }
    }
}

impl PeerListener {
    fn new(rx: mpsc::UnboundedReceiver<Envelope>, state_handler: Address<StateHandler>) -> Self {
        Self { rx, state_handler }
    }

}
