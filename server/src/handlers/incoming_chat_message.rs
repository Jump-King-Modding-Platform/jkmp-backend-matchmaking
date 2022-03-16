use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{
    chat::ChatChannel,
    client::Client,
    codec::MessagesCodec,
    messages::{IncomingChatMessage, Message, OutgoingChatMessage},
    state::State,
    util::string::truncate,
};

pub async fn handle_message(
    message: &IncomingChatMessage,
    _messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    // Trim the incoming message from whitespace and limit it to 100 characters
    let trimmed_message = truncate(message.message.trim(), 100);

    // Ignore empty messages
    if trimmed_message.is_empty() {
        return Ok(());
    }

    let state = state.lock().await;
    let client = state.get_client(source).context("Client not found")?;

    tracing::info!(
        "[{:?}] <{}> {}",
        message.channel,
        client.name,
        message.message
    );

    // List of clients to send the message to
    let mut target_clients = Vec::<&Client>::new();

    match message.channel {
        ChatChannel::Global => {
            for other_client in state.get_clients_iter() {
                target_clients.push(other_client.1);
            }
        }
        ChatChannel::Group => {
            for other_client in state.get_clients_in_group(state.get_matchmaking_options(source)) {
                target_clients.push(other_client);
            }
        }
        _ => anyhow::bail!("Unexpected channel"),
    }

    let outgoing_chat_message = OutgoingChatMessage {
        channel: message.channel,
        message: trimmed_message.to_string(),
        sender_id: Some(client.steam_id),
        sender_name: Some(client.name.clone()),
    };

    for other_client in target_clients {
        // Ignore errors from failing to send message to receiver
        let _ = other_client.send(Message::OutgoingChatMessage(outgoing_chat_message.clone()));
    }

    Ok(())
}
