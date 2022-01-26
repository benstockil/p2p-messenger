mod actors;
mod objects;
mod request;

use crate::request::Request;
use crate::actors::StateHandler;
use crate::actors::PeerListener;

#[tokio::main]
async fn main() {
    let state_handler = StateHandler::new();
    let listener = PeerListener::new();

    state_handler.send(
        Request::Debug("hi".into())
    ).await;

    state_handler.stop().await;
}
