use tokio::sync::oneshot;

use super::Request;

pub struct Envelope<T: Request> {
    pub request: T,
    pub response_channel: oneshot::Sender<T::Response>,
}
