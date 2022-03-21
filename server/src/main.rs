use server::{listener::Listener, steam::SteamAuthBackend};
use structopt::StructOpt;
use tokio::sync::mpsc;

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

    Listener::default()
        .host(options.host)
        .port(options.port)
        .listen::<SteamAuthBackend>(mpsc::channel::<()>(1).1)
        .await?;

    Ok(())
}
