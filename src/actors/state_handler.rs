use std::collections::HashMap;
use async_trait::async_trait;
use tokio::io;
use tokio::sync::{mpsc, oneshot};
use crate::actors::{PeerListener, ClientHandler};
use crate::objects::{Group, GroupId, Message, Peer, User, UserId};
use crate::actor::{Actor, Address, Envelope};


#[derive(Debug)]
pub struct StateHandler {
    users: HashMap<UserId, User>,
    peers: HashMap<UserId, Address<ClientHandler>>,
    listener: Address<PeerListener>,
    groups: HashMap<GroupId, Group>,
    rx: mpsc::UnboundedReceiver<Envelope>,
    messages: Vec<Message>,
}

#[async_trait]
impl Actor for StateHandler {
    async fn run(&mut self) -> io::Result<()> {
        let mut run = true;
        while run {
            let (incoming, response_channel) = self.rx.recv().await.unwrap();
            match incoming {
                // Request::Stop => { run = false; },
                Request::NewClient(client_handler) => {
                    self.peers.push(client_handler);
                }
                _ => { dbg!(incoming); }
            }
            response_channel.send(InternalResponse::Ok).unwrap();
        }
        Ok(())
    }
}

impl StateHandler {
    fn new(rx: mpsc::UnboundedReceiver<Envelope>) -> Self {
        Self {
            rx,
            peers: HashMap::new(),
            users: HashMap::new(),
            groups: HashMap::new(),
            listener: None,
            messages: Vec::new(),
        }
    }

    async fn new_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    async fn new_connection(&mut self, user_id: UserId, address: Address<PeerListener>) {
        self.peers.insert(user_id, address);
    }

    async fn new_group(&mut self, group: Group) {
        self.groups.insert(group.id.clone(), group);
    }

    async fn broadcast_message(&self, message: Message) {
        let group = self.groups.get(message.group).unwrap();
        group
            .members
            .iter()
            .map(|id| self.peers.get(id)?.send(Request::Message(message)))
    }

    async fn receive_message(&mut self, message: Message) {
        self.messages.push(message);
        self.broadcast_message(message).await;
    }
}
pub enum Request {
    NewClient(ClientHandler),
    Message(Message),
    Debug(String),
}
