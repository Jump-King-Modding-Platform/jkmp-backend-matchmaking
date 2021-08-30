use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub steam_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub success: bool,
}
