use tokio::net::TcpStream;
use crate::objects::Message;
use crate::request::{
    InternalResponse,
};
use crate::actors::{StateHandler, ClientHandlerActor};
use super::actor::ActorHandle;
use super::client_handler_actor::{ClientHandlerConfig, Request};

#[derive(Clone, Debug)]
pub struct ClientHandler {
    handle: ActorHandle<ClientHandlerActor>
}

impl ClientHandler {
    pub fn new(stream: TcpStream, state_handler: StateHandler) -> Self {
        let config = ClientHandlerConfig { stream, state_handler };
        let handle = ActorHandle::run(config);
        Self { handle }
    }

    pub async fn send_msg(&self, message: Message) -> InternalResponse {
        self.handle.call(Request::Message(message)).await
    }
}
