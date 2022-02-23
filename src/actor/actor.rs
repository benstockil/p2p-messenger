use async_trait::async_trait;
use tokio::io;
use tokio::sync::mpsc;
use super::{Address, Envelope};

#[async_trait]
pub trait Actor: Send {
    fn start(&self) -> Address<Self> {
        ActorHandle::run(self)
    }

    async fn run(&mut self) -> io::Result<()>;
}

#[derive(Debug)]
pub struct ActorHandle<T: Actor> {
    tx: mpsc::UnboundedSender<Envelope>,
}

impl<T: Actor> ActorHandle<T> {
    pub fn run(actor: T) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        
        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            actor.run().await.unwrap();
        });

        Self { tx }
    }

}

impl<T: Actor> Clone for ActorHandle<T> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}
