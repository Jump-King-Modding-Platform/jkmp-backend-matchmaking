use futures::{SinkExt, StreamExt};
use handlers::handshake;
use std::{net::SocketAddr, sync::Arc};
use structopt::StructOpt;
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
    sync::{mpsc, Mutex},
};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_util::codec::Decoder;

mod codec;
use codec::MessagesCodec;

mod messages;
use messages::Message;

mod state;
use state::State;

use crate::messages::ServerStatusUpdate;

mod client;

mod handlers;

mod chat;
mod encoding;
mod math;
mod steam;
mod util;

type MessageType = Message;

#[derive(StructOpt)]
#[structopt(
    name = "JKMP Matchmaking Server",
    about = "Handles matchmaking between players"
)]
struct LaunchOptions {
    #[structopt(short, long, default_value = "0.0.0.0")]
    host: String,

    #[structopt(short, long)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();

    let options = LaunchOptions::from_args();

    if std::env::var("STEAM_API_KEY").is_err() {
        anyhow::bail!("Environment variable STEAM_API_KEY is missing");
    }

    let listener = TcpListener::bind(format!("{}:{}", options.host, options.port)).await?;
    let state = Arc::new(Mutex::new(State::new()));

    tracing::info!(
        "Server started, listening for clients on {}:{}",
        options.host,
        options.port
    );

    let mut scheduler = JobScheduler::new();
    let scheduler_state = state.clone();

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
                Err(error) => tracing::info!("An error occurred when accepting socket: {}", error),
                Ok((socket, address)) => {
                    let state = state.clone();
                    tokio::spawn(async move {
                        process_client(socket, address, state).await;
                    });
                }
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    tracing::info!("Server shutting down...");

    // todo: send message about shutdown to clients?

    Ok(())
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
async fn process_client(socket: TcpStream, address: SocketAddr, state: Arc<Mutex<State>>) {
    let (tx, mut rx) = mpsc::unbounded_channel::<MessageType>();
    let mut messages = MessagesCodec::new().framed(socket);

    match messages.next().await {
        Some(Ok(message)) => match message {
            Message::HandshakeRequest(request) => {
                if let Err(error) =
                    handshake::handle_message(&request, tx, &mut messages, &address, &state).await
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
