use tokio::io;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::request::{ReqRecvPair, Request, Response};

pub struct StateHandler {
    tx: Sender<ReqRecvPair>, 
}

impl StateHandler {
    pub async fn new() -> Self {
        let (tx, rx): (Sender<ReqRecvPair>, Receiver<ReqRecvPair>) = mpsc::channel(30);

        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            Self::run(rx).await.unwrap();
        });

        Self { tx }
    }

    pub async fn run(mut rx: Receiver<ReqRecvPair>) -> io::Result<()> {
        loop {
            let (incoming, response_channel) = rx.recv().await.unwrap();
            dbg!(incoming);
            response_channel.send(Response::Ok).await.unwrap();
        }
    }

    pub async fn send(&self, request: Request) -> Response {
        let (mut tx, mut rx) = mpsc::channel(1);
        let req_recv = (request, tx);
        self.tx.send(req_recv).await.unwrap();

        rx.recv().await.unwrap()
    }
}
