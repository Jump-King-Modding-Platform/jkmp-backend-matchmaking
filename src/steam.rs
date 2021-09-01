use std::fmt::Display;

use anyhow::Context;
use reqwest::StatusCode;

const APP_ID: u32 = 1061090;
const URL_AUTH_USER_TICKET: &str =
    "https://api.steampowered.com/ISteamUserAuth/AuthenticateUserTicket/v1/";

#[derive(Debug, serde::Deserialize)]
struct ApiResponse<T> {
    response: Response<T>,
}

#[derive(Debug, serde::Deserialize)]
struct Response<T> {
    params: Option<T>,
    error: Option<ErrorResponse>,
}

#[derive(Debug, serde::Deserialize)]
struct ErrorResponse {
    #[serde(rename = "errorcode")]
    error_code: i32,
    #[serde(rename = "errordesc")]
    error_desc: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateUserTicketParams {
    result: String,
    #[serde(rename = "steamid")]
    steam_id: String,
    #[serde(rename = "ownersteamid")]
    owner_steam_id: String,
    #[serde(rename = "vacbanned")]
    vac_banned: bool,
    #[serde(rename = "publisherbanned")]
    publisher_banned: bool,
}

#[derive(Debug)]
pub struct UserSteamId {
    pub steam_id: u64,
    pub owner_steam_id: u64,
}

impl Display for UserSteamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(SteamId: {}, OwnerId: {})",
            &self.steam_id, &self.owner_steam_id
        )
    }
}

impl UserSteamId {
    pub fn new(steam_id: u64, owner_steam_id: u64) -> Self {
        Self {
            steam_id,
            owner_steam_id,
        }
    }
}

/// Verifies the user auth ticket and if successful return the user steam id
pub async fn verify_user_auth_ticket(ticket: &[u8]) -> Result<UserSteamId, anyhow::Error> {
    let steam_api_key = std::env::var("STEAM_API_KEY")
        .context("Could not find STEAM_API_KEY environment variable")?;
    let client = reqwest::Client::builder()
        .user_agent("JKMP_BACKEND")
        .build()?;
    let ticket_str: String = hex::encode(&ticket);

    let response = client
        .get(URL_AUTH_USER_TICKET)
        .query(&[("key", &steam_api_key)])
        .query(&[("appid", APP_ID)])
        .query(&[("ticket", &ticket_str)])
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => {
            let response = response
                .json::<ApiResponse<AuthenticateUserTicketParams>>()
                .await
                .context("Unexpected api response from steam api")?;

            if let Some(params) = response.response.params {
                let steam_id = params.steam_id.parse::<u64>()?;
                let owner_steam_id = params.owner_steam_id.parse::<u64>()?;
                return Ok(UserSteamId::new(steam_id, owner_steam_id));
            }

            if let Some(error) = response.response.error {
                anyhow::bail!(
                    "Response was not successful: {} (error code {})",
                    error.error_desc,
                    error.error_code
                );
            }

            unreachable!();
        }
        default => anyhow::bail!("Unexpected response: {}", default),
    }
}
