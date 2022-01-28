use std::net::IpAddr;
use tokio::sync::{mpsc, oneshot};
use tokio::{io, net::TcpListener};
use crate::actors::{ClientHandler, StateHandler};
use crate::objects::Peer;
use crate::request::{IRRPair, InternalRequest, InternalResponse};

#[derive(Clone)]
pub struct PeerListener {
    tx: mpsc::UnboundedSender<IRRPair>,
}

impl PeerListener {
    pub fn new(state_handler: StateHandler) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            let mut actor = PeerListenerActor { state_handler, rx };
            actor.run().await.unwrap();
        });

        Self { tx }
    }

    pub async fn send(&self, request: InternalRequest) -> InternalResponse {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();

        rx.await.unwrap()
    }
}

struct PeerListenerActor {
    state_handler: StateHandler,
    rx: mpsc::UnboundedReceiver<IRRPair>,
}

impl PeerListenerActor {
    async fn run(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3030").await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let client_handler = ClientHandler::new(socket, self.state_handler.clone());
            self.state_handler.send(
                InternalRequest::NewClient(Peer { address: "192.168.0.43".parse().unwrap() })
            );
        }
    }
}
