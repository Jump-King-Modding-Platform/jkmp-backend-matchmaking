use std::{net::SocketAddr, sync::Arc};

use tokio::{net::TcpStream, sync::Mutex};
use tokio_util::codec::Framed;

use crate::{
    codec::MessagesCodec,
    messages::SetMatchmakingPassword,
    state::{MatchmakingOptions, State},
};

use super::MessageHandler;

#[async_trait::async_trait]
impl MessageHandler for SetMatchmakingPassword {
    async fn handle_message(
        &self,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error> {
        let mut state = state.lock().await;
        let level_name = state.get_matchmaking_options(source).level_name.clone();
        state.set_matchmaking_options(
            source,
            Some(MatchmakingOptions::new(self.password.clone(), level_name)),
        );
        Ok(())
    }
}
