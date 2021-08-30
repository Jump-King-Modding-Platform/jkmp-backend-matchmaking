use tokio::sync::mpsc;

use crate::MessageType;

pub struct Client {
    pub tx: mpsc::UnboundedSender<MessageType>,
    pub steam_id: u64,
    pub name: String,
}

impl Client {
    pub fn new(tx: mpsc::UnboundedSender<MessageType>) -> Self {
        Self {
            tx,
            steam_id: 0,
            name: String::default(),
        }
    }
}
