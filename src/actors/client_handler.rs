use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};

use crate::objects::Message;
use crate::request::{
    InternalRequest, 
    InternalResponse,
    IRRPair, 
    RemoteRequest,
};
use crate::actors::StateHandler;

#[derive(Clone)]
pub struct ClientHandler {
    tx: mpsc::UnboundedSender<IRRPair>,
}

impl ClientHandler {
    pub fn new(mut stream: TcpStream, state_handler: StateHandler) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Spawn the listening process as an async task 
        tokio::spawn(async move {
            let mut actor = ClientHandlerActor { 
                rx, 
                stream, 
                state_handler: &state_handler,
            };
            actor.run().await.unwrap();
        });

        Self { tx }
    }

    async fn send(&self, request: InternalRequest) -> InternalResponse {
        let (tx, rx) = oneshot::channel();
        let req_recv = (request, tx);
        self.tx.send(req_recv).unwrap();

        rx.await.unwrap()
    }

    pub async fn send_msg(&self, message: Message) -> InternalResponse {
        self.send(InternalRequest::Message(message)).await
    }

    pub async fn close_connection(&self) -> InternalResponse {
        self.send(InternalRequest::Stop).await
    }
}

struct ClientHandlerActor<'a> {
    rx: mpsc::UnboundedReceiver<IRRPair>,
    stream: TcpStream,
    state_handler: &'a StateHandler,
}

impl<'a> ClientHandlerActor<'a> {
    pub async fn run(&mut self) -> io::Result<()> {
        tokio::select! {
            _ = self.stream.readable() => {
                let mut buf = Vec::new();
                self.stream.read_to_end(&mut buf).await?;

                let request: RemoteRequest = serde_json::from_slice(&buf).unwrap();
                self.handle_remote_request(request).await;
            }
            // state_handler.send(InternalRequest::NewClient(Peer { address: IpAddr::V4("192.168.0.43") }))

            incoming = self.rx.recv() => {
                let (request, tx) = incoming.unwrap();
                self.handle_internal_request(request, tx).await;
            }
        };
        Ok(())
    }

    async fn handle_remote_request(&self, request: RemoteRequest) {
        match request {
            RemoteRequest::Message(msg) => {
                self.state_handler.send(InternalRequest::Message(msg)).await;
            }
        }
    }

    async fn handle_internal_request(&mut self, request: InternalRequest, tx: oneshot::Sender<InternalResponse>) {
        match request {
            InternalRequest::Message(msg) => {
                self.send_remote_request(RemoteRequest::Message(msg)).await;
            }
            _ => {},
        }
    }

    async fn send_remote_request(&mut self, request: RemoteRequest) {
        // TODO Implement RemoteResponse type        
        let serialized = serde_json::to_vec(&request).unwrap();
        self.stream.write_all(serialized.as_slice()).await.unwrap();
    }
}
