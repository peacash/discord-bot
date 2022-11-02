mod commands;
use clap::Parser;
use pea_api::get;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::thread;
use std::time::Duration;
const HTTP_API: &str = "http://localhost:8080";
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Discord bot auth token
    #[clap(short, long)]
    pub token: String,
}
struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "height" => commands::height::run(&command.data.options).await,
                "hash" => commands::hash::run(&command.data.options).await,
                "block" => commands::block::run(&command.data.options).await,
                _ => "not implemented :(".to_string(),
            };
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::height::register(command))
                .create_application_command(|command| commands::hash::register(command))
                .create_application_command(|command| commands::block::register(command))
        })
        .await
        .unwrap();
        let mut i = 0;
        loop {
            i += 1;
            let activity = match i {
                1 => {
                    let height = match get::height(HTTP_API).await {
                        Ok(height) => height.to_string(),
                        Err(_) => "Unknown".to_string(),
                    };
                    Activity::playing(format!("{} blocks", height))
                }
                2 => Activity::playing("https://pea.cash"),
                _ => {
                    i = 0;
                    Activity::playing("github.com/peacash")
                }
            };
            ctx.set_activity(activity).await;
            thread::sleep(Duration::from_secs(3));
        }
    }
}
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut client = Client::builder(args.token, GatewayIntents::empty()).event_handler(Handler).await.expect("Error creating client");
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
