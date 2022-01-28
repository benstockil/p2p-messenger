use tokio::io;
use tokio::sync::{mpsc, oneshot};

use crate::request::{
    InternalRequest,
    InternalResponse,
    IRRPair,
};
use crate::actors::{ClientHandler, PeerListener};

#[derive(Clone)]
pub struct StateHandler {
    tx: mpsc::UnboundedSender<IRRPair>, 
}

impl StateHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            let mut actor = StateHandlerActor {
                peers: Vec::new(),
                listener: None,
                rx,
            };
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

    pub async fn stop(&self) {
        let (tx, _) = oneshot::channel();
        let req_recv = (InternalRequest::Stop, tx);
        self.tx.send(req_recv).unwrap();
    }
}

struct StateHandlerActor {
    peers: Vec<ClientHandler>,
    listener: Option<PeerListener>,
    rx: mpsc::UnboundedReceiver<IRRPair>,
}

impl StateHandlerActor {
    async fn run(&mut self) -> io::Result<()> {
        let mut run = true;
        while run {
            let (incoming, response_channel) = self.rx.recv().await.unwrap();
            match incoming {
                InternalRequest::Stop => { 
                    run = false; 
                    println!("stop!!");
                },
                _ => { dbg!(incoming); }
            }
            dbg!(response_channel.send(InternalResponse::Ok));
        }
        Ok(())
    }
}
