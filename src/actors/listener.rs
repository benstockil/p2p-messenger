use crate::actors::{PeerListenerActor, StateHandler};
use super::{
    actor::ActorHandle, 
    listener_actor::PeerListenerConfig,
};

#[derive(Clone, Debug)]
pub struct PeerListener {
    handle: ActorHandle<PeerListenerActor>,
}

impl PeerListener {
    pub fn new(state_handler: StateHandler) -> Self {
        let config = PeerListenerConfig { state_handler };
        let handle = ActorHandle::run(config);
        Self { handle }
    }
}
