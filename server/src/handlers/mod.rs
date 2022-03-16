use std::{net::SocketAddr, sync::Arc};

use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{codec::MessagesCodec, messages::Message, state::State};

pub mod handshake;
pub mod incoming_chat_message;
pub mod position_update;
pub mod set_matchmaking_password;

pub async fn handle_message(
    message: &Message,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    tracing::trace!("handling message: {:?}", message);
    match message {
        Message::PositionUpdate(val) => {
            position_update::handle_message(val, messages, source, state).await
        }
        Message::SetMatchmakingPassword(val) => {
            set_matchmaking_password::handle_message(val, messages, source, state).await
        }
        Message::IncomingChatMessage(val) => {
            incoming_chat_message::handle_message(val, messages, source, state).await
        }
        _ => anyhow::bail!("Unexpected message"),
    }
}
