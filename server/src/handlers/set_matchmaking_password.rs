use std::{net::SocketAddr, sync::Arc};

use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{
    codec::MessagesCodec,
    messages::SetMatchmakingPassword,
    state::{MatchmakingOptions, State},
};

pub async fn handle_message(
    message: &SetMatchmakingPassword,
    messages: &mut Framed<TcpStream, MessagesCodec>,
    source: &SocketAddr,
    state: &Arc<Mutex<State>>,
) -> Result<(), anyhow::Error> {
    let mut state = state.lock().await;
    let level_name = state.get_matchmaking_options(source).level_name.clone();
    let matchmaking_options = MatchmakingOptions::new(message.password.clone(), level_name);
    state.set_matchmaking_options(source, Some(matchmaking_options.clone()));

    let client = state.get_client(source).unwrap();
    let nearby_clients = state.get_nearby_clients(&client.position, &matchmaking_options);

    if nearby_clients.len() > 1 {
        crate::util::networking::send_nearby_clients(&client.steam_id, messages, &nearby_clients)
            .await?;
    }

    Ok(())
}
