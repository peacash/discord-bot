use crate::bot::Bot;
use crate::util;
use crate::EMBED_COLOR;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::Context;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    let height: usize = reqwest::get(format!("{}/height", bot.api)).await.unwrap().json().await.unwrap();
    command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|e| {
                        e.color(EMBED_COLOR)
                            .title("Height")
                            .description(util::markdown_code_block("fix", &height.to_string()))
                    })
                })
        })
        .await
        .unwrap();
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("height").description("Get current height")
}
