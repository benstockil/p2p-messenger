mod actors;
mod objects;
mod request;

use crate::request::InternalRequest;
use crate::actors::StateHandler;
use crate::actors::PeerListener;

#[tokio::main]
async fn main() {
    let state_handler = StateHandler::new();
    let listener = PeerListener::new(state_handler.clone());

    state_handler.send(
        InternalRequest::Debug("hi".into())
    ).await;

    state_handler.stop().await;
}
