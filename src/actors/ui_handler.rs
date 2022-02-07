use async_trait::async_trait;
use tui::{backend::CrosstermBackend, Terminal};
use tokio::sync::{mpsc, oneshot};
use std::io;

use crate::actors::StateHandler;
use crate::actor::{Actor, Address};

type IRRPair = (Request, oneshot::Sender<Response>);

pub struct UiHandlerConfig {
    pub state_handler: Address<StateHandler>,
}

#[derive(Debug)]
pub enum Request {}

#[derive(Debug)]
pub enum Response {}

#[derive(Debug)]
pub struct UiHandler {
    rx: mpsc::UnboundedReceiver<IRRPair>,
    state_handler: StateHandler,
}

#[async_trait]
impl Actor for UiHandler {
    async fn run(&mut self) -> io::Result<()> {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        Ok(())
    }
}

impl UiHandler {
    fn new(rx: mpsc::UnboundedReceiver<IRRPair>, state_handler: Address<StateHandler>) -> Self {
        Self {
            rx,
            state_handler,
        }
    }
}
