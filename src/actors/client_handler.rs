use async_trait::async_trait;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, oneshot};
use crate::actor::{Actor, Address, Envelope, Request};
use crate::remote::RemoteRequest;

use super::StateHandler;

#[derive(Debug)]
pub struct ClientHandler {
    rx: mpsc::UnboundedReceiver<Envelope>,
    stream: TcpStream,
    state_handler: Address<StateHandler>,
}

#[async_trait]
impl Actor for ClientHandler {
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

impl ClientHandler {
    fn new(rx: mpsc::UnboundedReceiver<Envelope>, stream: TcpStream, state_handler: Address<StateHandler>) -> Self {
        Self { rx, stream, state_handler }
    }

    async fn handle_remote_request(&self, request: RemoteRequest) {
        match request {
            RemoteRequest::Message(msg) => {
                // self.state_handler.new_msg(msg).await;
            }
        }
    }

    async fn handle_internal_request(&mut self, request: Envelope) {
        match request {
            Request::Message(msg) => {
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
