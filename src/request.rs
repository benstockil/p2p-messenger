use tokio::sync::mpsc::Sender;
use tokio::net::TcpStream;

#[derive(Debug)]
pub enum Request {
    NewClient(TcpStream),
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Err,
}

pub type ReqRecvPair = (Request, Sender<Response>);
