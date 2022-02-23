use std::any::Any;
use tokio::sync::oneshot;

use super::Request;

struct EnvelopeGeneric<T: Request> {
    pub request: T,
    pub response_channel: oneshot::Sender<T::Response>,
}

pub type Envelope = EnvelopeGeneric<dyn Request<Response = dyn Any>>;
