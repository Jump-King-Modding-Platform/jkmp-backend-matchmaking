use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{codec::MessagesCodec, messages::PositionUpdate, state::State};

use super::MessageHandler;

#[async_trait::async_trait]
impl MessageHandler for PositionUpdate {
    async fn handle_message(
        &self,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error> {
        let steam_id: u64;
        let mut state = state.lock().await;
        {
            let mut client = state.get_client_mut(source).context("Client not found")?;
            client.position = self.position;
            steam_id = client.steam_id;
        }

        let matchmaking_options = state.get_matchmaking_options(source);
        let nearby_clients = state.get_nearby_clients(&self.position, matchmaking_options);

        if nearby_clients.len() > 1 {
            crate::util::networking::send_nearby_clients(&steam_id, messages, &nearby_clients)
                .await?;
        }

        Ok(())
    }
}
