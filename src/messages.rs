use serde::{Deserialize, Serialize};

use crate::{chat::ChatChannel, math::Vector2};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    HandshakeRequest(HandshakeRequest),
    HandshakeResponse(HandshakeResponse),
    PositionUpdate(PositionUpdate),
    SetMatchmakingPassword(SetMatchmakingPassword),
    InformNearbyClients(InformNearbyClients),
    IncomingChatMessage(IncomingChatMessage),
    OutgoingChatMessage(OutgoingChatMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub auth_session_ticket: Vec<u8>,
    pub name: String,
    pub matchmaking_password: Option<String>,
    pub level_name: String,
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
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InformNearbyClients {
    pub client_ids: Vec<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingChatMessage {
    pub message: String,
    pub channel: ChatChannel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingChatMessage {
    pub message: String,
    pub channel: ChatChannel,
    pub sender_name: Option<String>,
    pub sender_id: Option<u64>,
}
