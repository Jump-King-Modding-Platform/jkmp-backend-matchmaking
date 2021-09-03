use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
};
use tokio_util::codec::Framed;

use crate::{codec::MessagesCodec, messages::Message, state::State};

pub mod handshake;
pub mod position_update;
pub mod set_matchmaking_password;

#[async_trait::async_trait]
pub trait MessageHandler {
    async fn handle_message(
        &self,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
pub trait HandshakeRequestMessageHandler {
    async fn handle_message(
        &self,
        tx: mpsc::UnboundedSender<Message>,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error>;
}

#[async_trait::async_trait]
impl MessageHandler for Message {
    async fn handle_message(
        &self,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &SocketAddr,
        state: &Arc<Mutex<State>>,
    ) -> Result<(), anyhow::Error> {
        println!("handling message: {:?}", self);
        match self {
            Message::PositionUpdate(val) => val.handle_message(messages, source, state).await,
            Message::SetMatchmakingPassword(val) => {
                val.handle_message(messages, source, state).await
            }
            _ => anyhow::bail!("Unexpected message"),
        }
    }
}
