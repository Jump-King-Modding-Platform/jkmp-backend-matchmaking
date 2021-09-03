use serde::{Deserialize, Serialize};

use crate::math::Vector2;

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    PositionUpdate(PositionUpdate),
    SetMatchmakingPassword(SetMatchmakingPassword),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub auth_session_ticket: Vec<u8>,
    pub name: String,
    pub matchmaking_password: String,
    pub position: Vector2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub success: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PositionUpdate {
    pub position: Vector2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetMatchmakingPassword {
    pub password: String,
}
