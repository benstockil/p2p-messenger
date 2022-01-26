use tokio::io;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::request::{ReqRecvPair, Request, Response};

pub struct StateHandler {
    tx: mpsc::Sender<ReqRecvPair>, 
    process: JoinHandle<()>,
}

impl StateHandler {
    pub fn new() -> Self {
        let (tx, rx): (mpsc::Sender<ReqRecvPair>, mpsc::Receiver<ReqRecvPair>) = mpsc::channel(30);

        // Spawn the listening process as an async task 
        let process = tokio::spawn(async move {
            Self::run(rx).await.unwrap();
        });

        Self { tx, process }
    }

    pub async fn run(mut rx: mpsc::Receiver<ReqRecvPair>) -> io::Result<()> {
        let mut run = true;
        while run {
            let (incoming, response_channel) = rx.recv().await.unwrap();
            match incoming {
                Request::Stop => { run = false; },
                _ => { dbg!(incoming); }
            }
            response_channel.send(Response::Ok).unwrap();
        }
        Ok(())
    }

    pub async fn send(&self, request: Request) -> Response {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).await.unwrap();

        rx.await.unwrap()
    }

    pub async fn stop(&self) {
        let (tx, _) = oneshot::channel();
        let req_recv = (Request::Stop, tx);
        self.tx.send(req_recv).await.unwrap();
    }
}
