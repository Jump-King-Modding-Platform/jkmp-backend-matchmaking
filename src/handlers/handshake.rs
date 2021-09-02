use futures::SinkExt;
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::Framed;

use super::HandshakeRequestMessageHandler;
use crate::{
    client::Client,
    codec::MessagesCodec,
    messages::{HandshakeRequest, HandshakeResponse, Message},
    steam,
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
        match steam::verify_user_auth_ticket(&self.auth_session_ticket).await {
            Ok(ids) => {
                let client = Client::new(tx, ids.steam_id, String::default());
                state.lock().await.add_client(source, client);
                println!("{} connected with steam_id {}", source, ids);

                messages
                    .send(Message::HandshakeResponse(HandshakeResponse {
                        success: true,
                    }))
                    .await?;
            }
            Err(error) => {
                println!("{} failed to auth: {}", source, error);

                messages
                    .send(Message::HandshakeResponse(HandshakeResponse {
                        success: false,
                    }))
                    .await?;
            }
        }

        Ok(())
    }
}
