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
                let name = self.name.trim();

                if name.is_empty() {
                    return send_response(
                        messages,
                        HandshakeResponse {
                            success: false,
                            error_message: Some("Player name is empty".to_string()),
                        },
                    )
                    .await;
                }

                let client = Client::new(tx, ids.steam_id, name.into());
                println!("{} connected", client);
                state.lock().await.add_client(source, client);

                send_response(
                    messages,
                    HandshakeResponse {
                        success: true,
                        error_message: None,
                    },
                )
                .await?;
            }
            Err(error) => {
                println!("{} failed to auth: {}", source, error);

                send_response(
                    messages,
                    HandshakeResponse {
                        success: false,
                        error_message: Some(error.to_string()),
                    },
                )
                .await?;
            }
        }

        Ok(())
    }
}

#[inline]
async fn send_response(
    messages: &mut Framed<TcpStream, MessagesCodec>,
    response: HandshakeResponse,
) -> Result<(), anyhow::Error> {
    messages.send(Message::HandshakeResponse(response)).await?;
    Ok(())
}
