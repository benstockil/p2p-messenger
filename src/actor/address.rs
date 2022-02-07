use super::{Actor, Envelope};
use tokio::sync::{mpsc, oneshot};

trait Request {
    type Response;
}

pub struct Address<T: Actor> {
    tx: mpsc::UnboundedReceiver<Envelope>,
}

impl Address {
    pub async fn do_send<T: Request>(&self, request: T) {
        let (tx, _) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();
    }

    pub async fn send<T: Request>(&self, request: T) -> T::Response {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();

        rx.await.unwrap()
    }
}
