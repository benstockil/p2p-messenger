use crate::{actors::StateHandlerActor, objects::{Message, Peer}};
use super::{ClientHandler, actor::ActorHandle, state_handler_actor::{StateHandlerConfig, Request}};

#[derive(Clone, Debug)]
pub struct StateHandler {
    handle: ActorHandle<StateHandlerActor>,
}

impl StateHandler {
    pub fn new() -> Self {
        let config = StateHandlerConfig {};
        let handle = ActorHandle::run(config);
        Self { handle }
    }

    pub async fn debug(&self, message: String) {
       self.handle.call(Request::Debug(message)).await;
    }

    pub async fn new_client(&self, peer: ClientHandler) {
        self.handle.call(Request::NewClient(peer)).await;
    }

    pub async fn message(&self, message: Message) {
        self.handle.call(Request::Message(message)).await;
    }
}

