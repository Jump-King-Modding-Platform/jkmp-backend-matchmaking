use futures::{SinkExt, StreamExt};
use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    signal,
    sync::{mpsc, Mutex},
};
use tokio_util::codec::Decoder;

mod codec;
use codec::MessagesCodec;

mod messages;
use messages::Message;

mod state;
use state::State;

mod client;

mod handlers;
use handlers::MessageHandler;

mod steam;

use crate::handlers::HandshakeRequestMessageHandler;

type MessageType = Message;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let listener = TcpListener::bind("127.0.0.1:16000").await?;
    let state = Arc::new(Mutex::new(State::new()));

    loop {
        tokio::select! {
            result = listener.accept() => match result {
                Err(error) => println!("An error occurred when accepting socket: {}", error),
                Ok((socket, address)) => {
                    process_client(socket, address, state.clone()).await;
                }
            },
            _ = signal::ctrl_c() => {
                break;
            }
        }
    }

    println!("Server shutting down...");

    // todo: send message about shutdown to clients?

    Ok(())
}

async fn process_client(socket: TcpStream, address: SocketAddr, state: Arc<Mutex<State>>) {
    tokio::spawn(async move {
        let (tx, mut rx) = mpsc::unbounded_channel::<MessageType>();
        let mut messages = MessagesCodec::new().framed(socket);

        match messages.next().await {
            Some(Ok(message)) => match message {
                Message::HandshakeRequest(request) => {
                    if let Err(error) = request
                        .handle_message(tx, &mut messages, &address, &state)
                        .await
                    {
                        println!(
                            "An error occurred while handling handshake from {}: {:?}",
                            address, error
                        );
                        return;
                    }
                }
                _ => {
                    println!("Did not receive a valid handshake");
                    return;
                }
            },
            Some(Err(error)) => {
                println!(
                    "Error occurred while reading handshake from {}: {:?}",
                    address, error
                );
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
                        println!("Failed to send message to {}: {:?}", address, error);
                        break; // Client disconnected
                    }
                },
                result = messages.next() => match result {
                    Some(Ok(message)) => {
                        if let Err(error) = message.handle_message(&mut messages, &address, &state).await {
                            println!("An error occured when handling message from {}: {:?}", address, error);
                            break;
                        }
                    },
                    Some(Err(error)) => {
                        println!("An error occured when reading message from {}: {:?}", address, error);
                        break;
                    },
                    None => break // Client disconnected
                },
                else => break // Client disconnected
            }
        }

        state.lock().await.remove_client(&address);
        println!("{} disconnected", address);
    });
}
