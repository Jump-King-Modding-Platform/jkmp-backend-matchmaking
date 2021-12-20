use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{codec::MessagesCodec, messages::IncomingChatMessage, state::State};

use super::MessageHandler;

#[async_trait::async_trait]
impl MessageHandler for IncomingChatMessage {
    async fn handle_message(
        &self,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error> {
        let state = state.lock().await;
        let client = state.get_client(source).context("Client not found")?;

        todo!()
    }
}
