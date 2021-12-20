use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{codec::MessagesCodec, messages::IncomingChatMessage, state::State};

pub async fn handle_message(
    message: &IncomingChatMessage,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    let state = state.lock().await;
    let client = state.get_client(source).context("Client not found")?;

    match message.channel {
        crate::chat::ChatChannel::Global => todo!(),
        crate::chat::ChatChannel::Group => todo!(),
        _ => anyhow::bail!("Unexpected channel"),
    }
}
