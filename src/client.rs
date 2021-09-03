use std::fmt::Display;

use tokio::sync::mpsc::{self, error::SendError};

use crate::{math::Vector2, messages::Message, MessageType};
pub struct Client {
    tx: mpsc::UnboundedSender<MessageType>,
    pub steam_id: u64,
    pub name: String,
    pub matchmaking_password: String,
    pub position: Vector2,
}

impl Client {
    pub fn new(
        tx: mpsc::UnboundedSender<MessageType>,
        steam_id: u64,
        name: String,
        matchmaking_password: String,
        position: Vector2,
    ) -> Self {
        Self {
            tx,
            steam_id,
            name,
            matchmaking_password,
            position,
        }
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
