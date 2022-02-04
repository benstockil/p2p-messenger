use crate::actors::{PeerListenerActor, StateHandler};
use super::{actor::ActorHandle, listener_actor::PeerListenerConfig, ui_handler_actor::{UiHandlerActor, UiHandlerConfig}};

#[derive(Clone, Debug)]
pub struct UiHandler {
    handle: ActorHandle<UiHandlerActor>,
}

impl UiHandler {
    pub fn new(state_handler: StateHandler) -> Self {
        let config = UiHandlerConfig { state_handler };
        let handle = ActorHandle::run(config);
        Self { handle }
    }
}
