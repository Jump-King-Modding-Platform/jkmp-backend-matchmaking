use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait AuthBackend {
    async fn check_credentials() -> anyhow::Result<()>;
    async fn verify_auth_ticket(ticket: &[u8]) -> anyhow::Result<u64>;
    async fn get_player_names(ids: &[u64]) -> anyhow::Result<HashMap<u64, String>>;
}
