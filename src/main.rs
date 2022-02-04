mod actors;
mod objects;
mod request;

use crate::actors::UiHandler;
use crate::actors::StateHandler;
use crate::actors::PeerListener;

#[tokio::main]
async fn main() {
    let state_handler = StateHandler::new();
    let ui_handler = UiHandler::new(state_handler.clone());
    // let listener = PeerListener::new(state_handler.clone());

    state_handler.debug("hi!".into()).await; 

    std::thread::park();
}
