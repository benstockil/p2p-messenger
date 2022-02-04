use async_trait::async_trait;
use tui::{backend::CrosstermBackend, Terminal};
use tokio::sync::{mpsc, oneshot};
use std::io;

use crate::actors::StateHandler;
use super::actor::Actor;

type IRRPair = (Request, oneshot::Sender<Response>);

pub struct UiHandlerConfig {
    pub state_handler: StateHandler,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {}

#[derive(Debug)]
pub struct UiHandlerActor {
    rx: mpsc::UnboundedReceiver<IRRPair>,
    state_handler: StateHandler,
}

#[async_trait]
impl Actor for UiHandlerActor {
    type Request = Request;
    type Response = Response;
    type Config = UiHandlerConfig;

    fn new(rx: mpsc::UnboundedReceiver<IRRPair>, config: Self::Config) -> Self {
        Self {
            rx,
            state_handler: config.state_handler,
        }
    }

    async fn run(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        Ok(())
    }
}
