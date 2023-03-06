use clap::Parser;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use tofuri_bot::bot::Bot;
use tofuri_bot::Args;
use tofuri_core::*;
use tracing::error;
#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    tracing_subscriber::fmt::init();
    if args.dev {
        if args.api == HTTP_API {
            args.api = DEV_HTTP_API.to_string();
        }
    }
    let mut client = Client::builder(args.token, GatewayIntents::empty())
        .event_handler(Bot { api: args.api })
        .await
        .expect("Error creating client");
    if let Err(err) = client.start().await {
        error!("Client error: {:?}", err);
    }
}
