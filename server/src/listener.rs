use crate::{
    auth_backend::AuthBackend,
    handlers::{self, handshake},
    state::State,
};
use futures::{SinkExt, StreamExt};
use jkmp::{
    codec::MessagesCodec,
    messages::{Message, ServerStatusUpdate},
};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
    sync::{mpsc, Mutex},
};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_util::codec::Decoder;

pub struct Listener {
    host: String,
    port: u16,
    state: Arc<Mutex<State>>,
}

impl Default for Listener {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 16069,
            state: Default::default(),
        }
    }
}

impl Listener {
    pub fn host(mut self, host: String) -> Self {
        self.host = host;
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub async fn listen<AB: AuthBackend>(
        &mut self,
        mut cancel_rx: mpsc::Receiver<()>,
    ) -> anyhow::Result<()> {
        AB::check_credentials().await?;

        let listener = TcpListener::bind(format!("{}:{}", self.host, self.port)).await?;

        tracing::info!(
            "Server started, listening for clients on {}:{}",
            self.host,
            self.port
        );

        let mut scheduler = JobScheduler::new();
        let scheduler_state = self.state.clone();

        // Broadcast server status once every minute
        let broadcast_server_status_job = Job::new_async("1/60 * * * * *", move |_uuid, _l| {
            let state = scheduler_state.clone();
            Box::pin(async move {
                if let Err(error) = broadcast_server_update(state).await {
                    tracing::error!(
                        "An error occured while broadcasting server status: {}",
                        error
                    );
                }
            })
        })
        .unwrap();
        scheduler.add(broadcast_server_status_job).unwrap();

        scheduler.start();

        loop {
            tokio::select! {
                result = listener.accept() => match result {
                    Err(error) => {
                        tracing::error!("An error occurred when accepting socket: {}", error);
                    },
                    Ok((socket, address)) => {
                        let state = self.state.clone();
                        tokio::spawn(async move {
                            process_client::<AB>(socket, address, state).await;
                        });
                    }
                },
                _ = cancel_rx.recv() => {
                    break;
                }
                _ = signal::ctrl_c() => {
                    break;
                }
            }
        }

        tracing::info!("Server shutting down...");

        // todo: send message about shutdown to clients?

        Ok(())
    }
}

async fn broadcast_server_update(state: Arc<Mutex<State>>) -> Result<(), anyhow::Error> {
    let state = state.lock().await;

    let clients = state.get_clients_iter();
    let total_players = clients.len() as u32;

    if total_players == 0 {
        return Ok(());
    }

    for client in clients {
        let group_players = state
            .get_clients_in_group(state.get_matchmaking_options(client.0))
            .count() as u32;

        // Ignore failed sends
        let _ = client
            .1
            .send(Message::ServerStatusUpdate(ServerStatusUpdate {
                total_players,
                group_players,
            }));
    }

    Ok(())
}

#[tracing::instrument(skip(socket, state))]
async fn process_client<AB: AuthBackend>(
    socket: TcpStream,
    address: SocketAddr,
    state: Arc<Mutex<State>>,
) {
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
    let mut messages = MessagesCodec::default().framed(socket);

    match messages.next().await {
        Some(Ok(message)) => match message {
            Message::HandshakeRequest(request) => {
                if let Err(error) =
                    handshake::handle_message::<AB>(&request, tx, &mut messages, &address, &state)
                        .await
                {
                    tracing::warn!("An error occurred while handling handshake: {:?}", error);
                    return;
                }
            }
            _ => {
                tracing::warn!("Did not receive a valid handshake");
                return;
            }
        },
        Some(Err(error)) => {
            tracing::warn!("Error occurred while reading handshake: {:?}", error);
            return;
        }
        _ => {
            return;
        }
    }

    loop {
        tokio::select! {
            Some(outbound_message) = rx.recv() => {
                if let Err(error) = messages.send(outbound_message).await {
                    tracing::warn!("Failed to send message: {:?}", error);
                    break; // Client disconnected
                }
            },
            result = messages.next() => match result {
                Some(Ok(message)) => {
                    if let Err(error) = handlers::handle_message(&message, &mut messages, &address, &state).await {
                        tracing::warn!("An error occured when handling message: {:?}", error);
                        break;
                    }
                },
                Some(Err(error)) => {
                    tracing::warn!("An error occured when reading message: {:?}", error);
                    break;
                },
                None => break // Client disconnected
            },
            else => break // Client disconnected
        }
    }

    let mut state = state.lock().await;
    if let Some(client) = state.remove_client(&address) {
        tracing::info!("{} disconnected", client);
    }
}
