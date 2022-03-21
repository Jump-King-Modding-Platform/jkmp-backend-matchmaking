use std::collections::HashMap;

use async_trait::async_trait;
use jkmp::codec::MessagesCodec;
use server::{auth_backend::AuthBackend, listener::Listener};
use tokio::{net::TcpStream, sync::mpsc};
use tokio_util::codec::{Decoder, Framed};

pub const VALID_TICKET: &[u8] = b"ticket";

struct TestAuthBackend;

#[async_trait]
impl AuthBackend for TestAuthBackend {
    async fn check_credentials() -> anyhow::Result<()> {
        Ok(())
    }

    async fn verify_auth_ticket(ticket: &[u8]) -> anyhow::Result<u64> {
        if ticket != VALID_TICKET {
            anyhow::bail!("invalid ticket");
        }
        Ok(1)
    }

    async fn get_player_names(ids: &[u64]) -> anyhow::Result<HashMap<u64, String>> {
        let mut names = HashMap::new();
        for id in ids {
            names.insert(*id, format!("Gamer ID #{}", id));
        }
        Ok(names)
    }
}

pub async fn start_server(port: u16) -> anyhow::Result<mpsc::Sender<()>> {
    let (cancel_tx, cancel_rx) = mpsc::channel::<()>(1);

    tokio::spawn(async move {
        let _ = Listener::default()
            .port(port)
            .listen::<TestAuthBackend>(cancel_rx)
            .await;
    });

    Ok(cancel_tx)
}

pub async fn connect_client(port: u16) -> anyhow::Result<Framed<TcpStream, MessagesCodec>> {
    let socket = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;
    let messages = MessagesCodec::default().framed(socket);
    Ok(messages)
}
