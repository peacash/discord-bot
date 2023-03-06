use crate::commands;
use log::info;
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Activity;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::time::Duration;
pub struct Bot {
    pub api: String,
}
#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            match command.data.name.as_str() {
                "height" => commands::height::run(self, &ctx, &command).await,
                "hash" => commands::hash::run(self, &ctx, &command).await,
                "block" => commands::block::run(self, &ctx, &command).await,
                "balance" => commands::balance::run(self, &ctx, &command).await,
                "transaction" => commands::transaction::run(self, &ctx, &command).await,
                "stake" => commands::stake::run(self, &ctx, &command).await,
                _ => {}
            };
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::height::register(command))
                .create_application_command(|command| commands::hash::register(command))
                .create_application_command(|command| commands::block::register(command))
                .create_application_command(|command| commands::balance::register(command))
                .create_application_command(|command| commands::transaction::register(command))
                .create_application_command(|command| commands::stake::register(command))
        })
        .await
        .unwrap();
        loop {
            let height = match reqwest::get(format!("{}/height", self.api)).await {
                Ok(res) => res.json::<usize>().await.unwrap().to_string(),
                Err(_) => "?".to_string(),
            };
            ctx.set_activity(Activity::watching(format!("{} blocks", height))).await;
            tokio::time::sleep(Duration::from_secs(10)).await;
        }
    }
}
