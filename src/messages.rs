use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HandshakeRequest { steam_id: u64 },
    HandshakeResponse { success: bool },
}
