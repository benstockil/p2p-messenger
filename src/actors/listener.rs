use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::{io, net::TcpListener};

use crate::actors::ClientHandler;
use crate::request::{ReqRecvPair, Request, Response};

pub struct PeerListener {
    tx: Sender<ReqRecvPair>,
}

impl PeerListener {
    pub fn new() -> Self {
        let (tx, rx): (Sender<ReqRecvPair>, Receiver<ReqRecvPair>) = mpsc::channel(10);

        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            Self::run(rx).await.unwrap();
        });

        Self { tx }
    }

    pub async fn run(rx: Receiver<ReqRecvPair>) -> io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:3030").await?;

        loop {
            let (socket, _) = listener.accept().await?;
            let client_handler = ClientHandler::new(socket);
            state_handler.send(Request::NewClient(client_handler))
        }
    }

    pub async fn send(&self, request: Request) -> Response {
        let (tx, mut rx) = mpsc::channel(1);
        let req_recv = (request, tx);
        self.tx.send(req_recv).await.unwrap();

        rx.recv().await.unwrap()
    }
}
