use async_trait::async_trait;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use super::actor::Actor;
use crate::actors::StateHandler;
use crate::request::{InternalRequest, InternalResponse, RemoteRequest};

type RRPair = (
    <ClientHandlerActor as Actor>::Request,
    oneshot::Sender<<ClientHandlerActor as Actor>::Response>,
);

pub struct ClientHandlerConfig {
    pub stream: TcpStream,
    pub state_handler: StateHandler,
}

pub struct ClientHandlerActor {
    rx: mpsc::UnboundedReceiver<RRPair>,
    stream: TcpStream,
    state_handler: StateHandler,
}

#[async_trait]
impl Actor for ClientHandlerActor {
    type Request = InternalRequest;
    type Response = InternalResponse;
    type Config = ClientHandlerConfig;

    fn new(rx: mpsc::UnboundedReceiver<RRPair>, config: Self::Config) -> Self {
        Self {
            rx,
            stream: config.stream,
            state_handler: config.state_handler,
        }
    }

    async fn run(&mut self) -> io::Result<()> {
        tokio::select! {
            _ = self.stream.readable() => {
                let mut buf = Vec::new();
                self.stream.read_to_end(&mut buf).await?;

                let request: RemoteRequest = serde_json::from_slice(&buf).unwrap();
                self.handle_remote_request(request).await;
            }
            // state_handler.new_client(Peer { address: IpAddr::V4("192.168.0.43") });

            incoming = self.rx.recv() => {
                let (request, tx) = incoming.unwrap();
                self.handle_internal_request(request, tx).await;
            }
        };
        Ok(())
    }
}

impl ClientHandlerActor {
    async fn handle_remote_request(&self, request: RemoteRequest) {
        match request {
            RemoteRequest::Message(msg) => {
                // self.state_handler.new_msg(msg).await;
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
