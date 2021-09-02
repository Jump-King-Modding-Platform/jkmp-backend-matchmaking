use std::fmt::Display;

use tokio::sync::mpsc::{self, error::SendError};

use crate::{messages::Message, MessageType};
pub struct Client {
    tx: mpsc::UnboundedSender<MessageType>,
    pub steam_id: u64,
    pub name: String,
}

impl Client {
    pub fn new(tx: mpsc::UnboundedSender<MessageType>, steam_id: u64, name: String) -> Self {
        Self { tx, steam_id, name }
    }

    pub fn send(&self, message: Message) -> Result<(), SendError<Message>> {
        self.tx.send(message)
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", &self.name, &self.steam_id)
    }
}
