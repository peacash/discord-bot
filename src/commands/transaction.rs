use crate::bot::Bot;
use crate::util;
use crate::EMBED_COLOR;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Timestamp;
use serenity::prelude::Context;
use tofuri_api_core::Transaction;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(hash) = command
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap()
    {
        let transaction: Transaction = reqwest::get(format!("{}/transaction/{}", bot.api, hash))
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
                            e.color(EMBED_COLOR)
                                .timestamp(
                                    Timestamp::from_unix_timestamp(transaction.timestamp.into())
                                        .unwrap(),
                                )
                                .fields(vec![
                                    (
                                        "Input",
                                        util::markdown_code_block(
                                            "fix",
                                            &transaction.input_address,
                                        ),
                                        false,
                                    ),
                                    (
                                        "Output",
                                        util::markdown_code_block(
                                            "fix",
                                            &transaction.output_address,
                                        ),
                                        false,
                                    ),
                                    (
                                        "Amount",
                                        util::markdown_code_block(
                                            "diff",
                                            &format!("+ {}", transaction.amount),
                                        ),
                                        true,
                                    ),
                                    (
                                        "Fee",
                                        util::markdown_code_block(
                                            "diff",
                                            &format!("- {}", transaction.fee),
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
    command
        .name("transaction")
        .description("Get transaction by hash")
        .create_option(|option| {
            option
                .name("hash")
                .description("A hash")
                .kind(CommandOptionType::String)
                .min_int_value(0)
                .required(true)
        })
}
