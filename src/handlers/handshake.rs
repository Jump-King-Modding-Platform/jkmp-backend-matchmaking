use std::{net::SocketAddr, sync::Arc};

use futures::SinkExt;
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};
use tokio_util::codec::Framed;

use crate::{
    client::Client,
    codec::MessagesCodec,
    messages::{HandshakeRequest, HandshakeResponse, Message},
    state::{MatchmakingOptions, State},
    steam,
};

pub async fn handle_message(
    message: &HandshakeRequest,
    tx: mpsc::UnboundedSender<Message>,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    let name = message.name.trim();

    if name.is_empty() || name.len() < 2 || name.len() > 32 {
        return send_response(
            messages,
            HandshakeResponse {
                success: false,
                error_message: Some("Player name is invalid".to_string()),
            },
        )
        .await;
    }

    match steam::verify_user_auth_ticket(&message.auth_session_ticket).await {
        Ok(ids) => {
            let mut state = state.lock().await;
            let client = Client::new(tx, ids.steam_id, name.into(), message.position);
            println!("{} connected", client);

            let matchmaking_options = MatchmakingOptions::new(
                message.matchmaking_password.clone(),
                message.level_name.clone(),
            );
            state.add_client(source, client, matchmaking_options);

            send_response(
                messages,
                HandshakeResponse {
                    success: true,
                    error_message: None,
                },
            )
            .await?;
        }
        Err(error) => {
            println!("{} failed to auth: {}", source, error);

            send_response(
                messages,
                HandshakeResponse {
                    success: false,
                    error_message: Some(error.to_string()),
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
