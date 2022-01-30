use crate::{actors::StateHandlerActor, request::InternalRequest};
use super::{
    actor::ActorHandle, 
    state_handler_actor::StateHandlerConfig
};

#[derive(Clone)]
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
       self.handle.send(InternalRequest::Debug(message)).await;
    }
}

