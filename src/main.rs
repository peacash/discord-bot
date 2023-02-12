use clap::Parser;
use pea_bot::bot::Bot;
use pea_bot::Args;
use pea_core::*;
use serenity::prelude::GatewayIntents;
use serenity::Client;
#[tokio::main]
async fn main() {
    let mut args = Args::parse();
    if args.dev {
        if args.api == BIND_API {
            args.api = DEV_BIND_API.to_string();
        }
    }
    let mut client = Client::builder(args.token, GatewayIntents::empty())
        .event_handler(Bot { api: args.api })
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
