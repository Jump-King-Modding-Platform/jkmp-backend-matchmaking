use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::Framed;

use super::HandshakeRequestMessageHandler;
use crate::{
    client::Client,
    codec::MessagesCodec,
    messages::{HandshakeRequest, Message},
};

#[async_trait::async_trait]
impl HandshakeRequestMessageHandler for HandshakeRequest {
    async fn handle_message(
        &self,
        tx: mpsc::UnboundedSender<Message>,
        messages: &mut Framed<TcpStream, MessagesCodec>,
        source: &std::net::SocketAddr,
        state: &std::sync::Arc<tokio::sync::Mutex<crate::state::State>>,
    ) -> Result<(), anyhow::Error> {
        // todo: verify steam_id ownership and game ownership

        let client = Client::new(tx, self.steam_id, String::default());
        state.lock().await.add_client(source, client);
        println!("{} connected with steam_id {}", source, self.steam_id);

        Ok(())
    }
}
