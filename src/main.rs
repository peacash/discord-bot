use clap::Parser;
use serenity::prelude::GatewayIntents;
use serenity::Client;
use tofuri_bot::bot::Bot;
use tofuri_bot::Args;
use tracing::error;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload;
use tracing_subscriber::EnvFilter;
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let (layer, _) = reload::Layer::new(filter);
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE);
    let registry = tracing_subscriber::registry().with(layer);
    if args.without_time {
        registry.with(fmt_layer.without_time()).init();
    } else {
        registry.with(fmt_layer).init();
    }
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
