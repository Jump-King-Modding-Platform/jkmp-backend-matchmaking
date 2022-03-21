use jkmp::{math::Vector2, messages::Message};
use std::fmt::Display;
use tokio::sync::mpsc::{self, error::SendError};

pub const VERSION: u32 = 2;

pub struct Client {
    tx: mpsc::UnboundedSender<Message>,
    pub steam_id: u64,
    pub name: String,
    pub position: Vector2,
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.steam_id == other.steam_id
            && self.name == other.name
            && self.position == other.position
    }
}

impl Client {
    pub fn new(
        tx: mpsc::UnboundedSender<Message>,
        steam_id: u64,
        name: String,
        position: Vector2,
    ) -> Self {
        Self {
            tx,
            steam_id,
            name,
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
