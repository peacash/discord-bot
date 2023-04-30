use crate::bot::Bot;
use crate::util;
use crate::EMBED_COLOR;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::Integer(height) = command
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap()
    {
        let hash: String = reqwest::get(format!("{}/hash/{}", bot.api, height))
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(EMBED_COLOR).field(
                                format!("Hash - {}", height),
                                util::markdown_code_block("ini", &format!("[{}]", hash)),
                                false,
                            )
                        })
                    })
            })
            .await
            .unwrap();
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("hash")
        .description("Get hash by height")
        .create_option(|option| {
            option
                .name("height")
                .description("A positive integer")
                .kind(CommandOptionType::Integer)
                .min_int_value(0)
                .required(true)
        })
}
