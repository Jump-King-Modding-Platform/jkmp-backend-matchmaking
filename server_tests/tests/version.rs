use futures::{SinkExt, StreamExt};
use jkmp::{
    math::Vector2,
    messages::{HandshakeRequest, Message},
};
use server_tests::VALID_TICKET;

#[tokio::test]
async fn rejects_invalid_version() -> anyhow::Result<()> {
    let cancel_tx = server_tests::start_server(6960).await?;

    let mut client = server_tests::connect_client(6960).await?;
    client
        .send(Message::HandshakeRequest(HandshakeRequest {
            auth_session_ticket: VALID_TICKET.to_vec(),
            matchmaking_password: None,
            level_name: "game".to_string(),
            position: Vector2 { x: 0.0, y: 0.0 },
            version: 0,
        }))
        .await?;

    match client.next().await {
        Some(Ok(Message::HandshakeResponse(response))) => assert!(!response.success),
        Some(Ok(message)) => anyhow::bail!(
            "Expected server to return rejection message, got {:?}",
            message
        ),
        Some(Err(error)) => anyhow::bail!(
            "Expected server to return rejection message, got error: {:?}",
            error
        ),
        _ => anyhow::bail!("Expected server to return rejection message"),
    }

    cancel_tx.send(()).await?;
    Ok(())
}

#[tokio::test]
async fn rejects_invalid_ticket() -> anyhow::Result<()> {
    let cancel_tx = server_tests::start_server(6961).await?;

    let mut client = server_tests::connect_client(6961).await?;
    client
        .send(Message::HandshakeRequest(HandshakeRequest {
            auth_session_ticket: b"gamer".to_vec(),
            matchmaking_password: None,
            level_name: "game".to_string(),
            position: Vector2 { x: 0.0, y: 0.0 },
            version: 2,
        }))
        .await?;

    match client.next().await {
        Some(Ok(Message::HandshakeResponse(response))) => assert!(!response.success),
        Some(Ok(message)) => anyhow::bail!(
            "Expected server to return rejection message, got {:?}",
            message
        ),
        Some(Err(error)) => anyhow::bail!(
            "Expected server to return rejection message, got error: {:?}",
            error
        ),
        _ => anyhow::bail!("Expected server to return rejection message"),
    }

    cancel_tx.send(()).await?;
    Ok(())
}

#[tokio::test]
async fn accepts_valid_ticket() -> anyhow::Result<()> {
    let cancel_tx = server_tests::start_server(6962).await?;

    let mut client = server_tests::connect_client(6962).await?;
    client
        .send(Message::HandshakeRequest(HandshakeRequest {
            auth_session_ticket: VALID_TICKET.to_vec(),
            matchmaking_password: None,
            level_name: "game".to_string(),
            position: Vector2 { x: 0.0, y: 0.0 },
            version: 2,
        }))
        .await?;

    match client.next().await {
        Some(Ok(Message::HandshakeResponse(response))) => assert!(response.success),
        Some(Ok(message)) => anyhow::bail!(
            "Expected server to return rejection message, got {:?}",
            message
        ),
        Some(Err(error)) => anyhow::bail!(
            "Expected server to return rejection message, got error: {:?}",
            error
        ),
        _ => anyhow::bail!("Expected server to return rejection message"),
    }

    cancel_tx.send(()).await?;
    Ok(())
}
