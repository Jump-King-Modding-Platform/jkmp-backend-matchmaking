use crate::client::Client;
use futures::SinkExt;
use jkmp::{
    codec::MessagesCodec,
    messages::{InformNearbyClients, Message},
};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub async fn send_nearby_clients(
    except_steam_id: &u64,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    clients: &[&Client],
) -> Result<(), anyhow::Error> {
    let nearby_client_ids: Vec<u64> = clients
        .iter()
        .filter(|&&c| &c.steam_id != except_steam_id)
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

    Ok(())
}
