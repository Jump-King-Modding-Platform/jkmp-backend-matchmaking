use futures::{SinkExt, StreamExt};
use jkmp::{
    math::Vector2,
    messages::{HandshakeRequest, HandshakeResponse, Message},
};
use server_tests::{INVALID_TICKET, INVALID_VERSION, VALID_TICKET, VALID_VERSION};

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
            version: INVALID_VERSION,
        }))
        .await?;

    assert!(matches!(
        client.next().await,
        Some(Ok(Message::HandshakeResponse(HandshakeResponse {
            success: false,
            ..
        })))
    ));

    cancel_tx.send(()).await?;
    Ok(())
}

#[tokio::test]
async fn rejects_invalid_ticket() -> anyhow::Result<()> {
    let cancel_tx = server_tests::start_server(6961).await?;

    let mut client = server_tests::connect_client(6961).await?;
    client
        .send(Message::HandshakeRequest(HandshakeRequest {
            auth_session_ticket: INVALID_TICKET.to_vec(),
            matchmaking_password: None,
            level_name: "game".to_string(),
            position: Vector2 { x: 0.0, y: 0.0 },
            version: VALID_VERSION,
        }))
        .await?;

    assert!(matches!(
        client.next().await,
        Some(Ok(Message::HandshakeResponse(HandshakeResponse {
            success: false,
            ..
        })))
    ));

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
            version: VALID_VERSION,
        }))
        .await?;

    assert!(matches!(
        client.next().await,
        Some(Ok(Message::HandshakeResponse(HandshakeResponse {
            success: true,
            ..
        })))
    ));

    cancel_tx.send(()).await?;
    Ok(())
}
