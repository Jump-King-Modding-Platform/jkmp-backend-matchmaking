use anyhow::Context;
use futures::SinkExt;

use crate::messages::{InformNearbyClients, Message, PositionUpdate};

use super::MessageHandler;

#[async_trait::async_trait]
impl MessageHandler for PositionUpdate {
    async fn handle_message(
        &self,
        messages: &mut tokio_util::codec::Framed<
            tokio::net::TcpStream,
            crate::codec::MessagesCodec,
        >,
        source: &std::net::SocketAddr,
        state: &std::sync::Arc<tokio::sync::Mutex<crate::state::State>>,
    ) -> Result<(), anyhow::Error> {
        let mut state = state.lock().await;
        let mut client = state.get_client_mut(source).context("Client not found")?;
        client.position = self.position;
        let steam_id = client.steam_id;

        let nearby_clients = state.get_nearby_clients(&self.position);

        if nearby_clients.len() > 1 {
            let nearby_client_ids: Vec<u64> = nearby_clients
                .iter()
                .filter(|&&c| c.steam_id != steam_id)
                .map(|&c| c.steam_id)
                .collect();

            messages
                .send(Message::InformNearbyClients(InformNearbyClients {
                    client_ids: nearby_client_ids,
                }))
                .await?;
        }

        Ok(())
    }
}
