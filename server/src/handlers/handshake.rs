use crate::{
    auth_backend::AuthBackend,
    client::{self, Client},
    state::{MatchmakingOptions, State},
};
use anyhow::Context;
use futures::SinkExt;
use jkmp::{
    chat::ChatChannel,
    codec::MessagesCodec,
    messages::{
        HandshakeRequest, HandshakeResponse, Message, OutgoingChatMessage, ServerStatusUpdate,
    },
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};
use tokio_util::codec::Framed;

pub async fn handle_message<AB: AuthBackend>(
    message: &HandshakeRequest,
    tx: mpsc::UnboundedSender<Message>,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    if message.version != client::VERSION {
        send_response(
            messages,
            HandshakeResponse {
                success: false,
                error_message: Some("Your version is outdated".to_string()),
            },
        )
        .await?;
        anyhow::bail!("Client version {} mismatch", message.version);
    }

    match AB::verify_auth_ticket(&message.auth_session_ticket).await {
        Ok(steam_id) => {
            let user_names = AB::get_player_names(&[steam_id]).await?;
            let name = user_names
                .get(&steam_id)
                .context("Could not get user info from steam")?;

            let mut state = state.lock().await;
            let client = Client::new(tx, steam_id, name.clone(), message.position);
            tracing::info!("{} connected", client);

            let matchmaking_options = MatchmakingOptions::new(
                message.matchmaking_password.clone(),
                message.level_name.clone(),
            );

            let clients_total = state.get_clients_iter().len();
            let clients_in_group = state.get_clients_in_group(&matchmaking_options).count();

            let welcome_message = match (clients_total, clients_in_group) {
                (0, 0) => "Welcome! There are currently no other players online.".into(),
                (total, 0) => format!(
                    "Welcome! There are {} other players online, but none of them in your group.",
                    total
                ),
                (total, group) => format!(
                    "Welcome! There are {} other players online. {} of them are in your group.",
                    total, group
                ),
            };

            state.add_client(source, client, matchmaking_options);

            send_response(
                messages,
                HandshakeResponse {
                    success: true,
                    error_message: None,
                },
            )
            .await?;

            messages
                .send(Message::OutgoingChatMessage(OutgoingChatMessage {
                    channel: ChatChannel::Global,
                    sender_id: None,
                    sender_name: None,
                    message: welcome_message,
                }))
                .await?;

            messages
                .send(Message::ServerStatusUpdate(ServerStatusUpdate {
                    total_players: state.get_clients_iter().len() as u32,
                    group_players: state
                        .get_clients_in_group(state.get_matchmaking_options(source))
                        .count() as u32,
                }))
                .await?;
        }
        Err(error) => {
            tracing::info!("{} failed to auth: {}", source, error);

            send_response(
                messages,
                HandshakeResponse {
                    success: false,
                    error_message: Some(
                        "An unexpected error occured when handling handshake request".to_string(),
                    ),
                },
            )
            .await?;
        }
    }

    Ok(())
}

#[inline]
async fn send_response(
    messages: &mut Framed<TcpStream, MessagesCodec>,
    response: HandshakeResponse,
) -> Result<(), anyhow::Error> {
    messages.send(Message::HandshakeResponse(response)).await
}
