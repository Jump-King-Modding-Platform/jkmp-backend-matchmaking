use std::{net::SocketAddr, sync::Arc};

use anyhow::Context;
use futures::SinkExt;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{
    codec::MessagesCodec,
    messages::{InformNearbyClients, Message, PositionUpdate},
    state::State,
};

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
            let nearby_client_ids: Vec<u64> = nearby_clients
                .iter()
                .filter(|&&c| c.steam_id != steam_id)
                .map(|&c| c.steam_id)
                .collect();

            // Split up message into multiple messages if there's more than 50 clients to send
            for chunk in nearby_client_ids.chunks(50) {
                messages
                    .send(Message::InformNearbyClients(InformNearbyClients {
                        client_ids: chunk.into(),
                    }))
                    .await?;
            }
        }

        Ok(())
    }
}
