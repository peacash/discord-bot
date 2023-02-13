use crate::bot::Bot;
use crate::util;
use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use serenity::utils::Color;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(address) = command
        .data
        .options
        .get(0)
        .expect("Expected address option")
        .resolved
        .as_ref()
        .expect("Expected address object")
    {
        let balance = get::balance(&bot.api, address).await.unwrap();
        let staked = get::staked(&bot.api, address).await.unwrap();
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(Color::from_rgb(47, 49, 54)).fields(vec![
                                ("Address", util::markdown_code_block("fix", address), false),
                                ("Balance", util::markdown_code_block("diff", &format!("+ {}", balance)), true),
                                ("Staked", util::markdown_code_block("diff", &format!("- {}", staked)), true),
                            ])
                        })
                    })
            })
            .await
            .unwrap();
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("balance").description("Get balance of address").create_option(|option| {
        option
            .name("address")
            .description("An address")
            .kind(CommandOptionType::String)
            .min_int_value(0)
            .required(true)
    })
}
