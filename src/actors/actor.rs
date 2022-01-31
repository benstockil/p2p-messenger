use async_trait::async_trait;
use std::fmt::Debug;
use tokio::{io, sync::{mpsc, oneshot}};

type RRPair<T> = (<T as Actor>::Request, oneshot::Sender<<T as Actor>::Response>);

#[async_trait]
pub trait Actor: Send {
    type Request: Send + Debug;
    type Response: Send + Debug;
    type Config: Send;

    fn new(rx: mpsc::UnboundedReceiver<RRPair<Self>>, config: Self::Config) -> Self;

    async fn run(&mut self) -> io::Result<()>;
}

pub struct ActorHandle<T: Actor> {
    tx: mpsc::UnboundedSender<RRPair<T>>,
}

impl<T: Actor + 'static> ActorHandle<T> {
    pub fn run(config: T::Config) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            let mut actor = T::new(rx, config);
            actor.run().await.unwrap();
        });

        Self { tx }
    }

    pub async fn send(&self, request: T::Request) {
        let (tx, _) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();
    }

    pub async fn call(&self, request: T::Request) -> T::Response {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();

        rx.await.unwrap()
    }
}

impl<T: Actor> Clone for ActorHandle<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone()
        }
    }
}
