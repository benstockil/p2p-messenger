use std::marker::PhantomData;

use super::{Actor, Envelope};
use tokio::sync::{mpsc, oneshot};

trait Request {
    type Response: 'static;
}

trait Handle<R: Request> {}

pub struct Address<T: Actor> {
    phantom: PhantomData<T>,
    tx: mpsc::UnboundedReceiver<Envelope>,
}

impl<T: Actor> Address<T> {
    pub async fn do_send<R: Request>(&self, request: R) 
    where T: Handle<R>
    {
        let (tx, _) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();
    }

    pub async fn send<R: Request>(&self, request: R) -> R::Response {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();

        rx.await.unwrap()
    }
}
