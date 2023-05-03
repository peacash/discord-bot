use clap::Parser;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use tofuri_bot::bot::Bot;
use tofuri_bot::Args;
use tracing::error;
#[tokio::main]
async fn main() {
    let args = Args::parse();
    tracing_subscriber::fmt::init();
    let mut client = Client::builder(args.token, GatewayIntents::empty())
        .event_handler(Bot {
            api: args.api.to_string(),
        })
        .await
        .expect("Error creating client");
    if let Err(err) = client.start().await {
        error!("Client error: {:?}", err);
    }
}
