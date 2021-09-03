use crate::messages::PositionUpdate;

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
        todo!()
    }
}
