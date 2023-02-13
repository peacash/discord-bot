use crate::bot::Bot;
use crate::util;
use crate::EMBED_COLOR;
use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Timestamp;
use serenity::prelude::Context;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(hash) = command
        .data
        .options
        .get(0)
        .expect("Expected address option")
        .resolved
        .as_ref()
        .expect("Expected address object")
    {
        let block = get::block(&bot.api, hash).await.unwrap();
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(EMBED_COLOR)
                                .timestamp(Timestamp::from_unix_timestamp(block.timestamp.into()).unwrap())
                                .fields(vec![
                                    ("Forger", util::markdown_code_block("fix", &block.address), false),
                                    (
                                        "Transactions",
                                        util::markdown_code_block(
                                            "diff",
                                            &format!("{} {}", if block.transactions.is_empty() { "-" } else { "+" }, block.transactions.len()),
                                        ),
                                        true,
                                    ),
                                    (
                                        "Stakes",
                                        util::markdown_code_block(
                                            "diff",
                                            &format!("{} {}", if block.stakes.is_empty() { "-" } else { "+" }, block.stakes.len()),
                                        ),
                                        true,
                                    ),
                                ])
                        })
                    })
            })
            .await
            .unwrap();
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("block").description("Get block by hash").create_option(|option| {
        option
            .name("hash")
            .description("A hash")
            .kind(CommandOptionType::String)
            .min_int_value(0)
            .required(true)
    })
}
