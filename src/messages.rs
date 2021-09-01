use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub auth_session_ticket: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub success: bool,
}
