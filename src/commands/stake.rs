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
    if let CommandDataOptionValue::String(hash) = command.data.options.get(0).unwrap().resolved.as_ref().unwrap() {
        let stake = get::stake(&bot.api, hash).await.unwrap();
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(EMBED_COLOR)
                                .timestamp(Timestamp::from_unix_timestamp(stake.timestamp.into()).unwrap())
                                .fields(vec![
                                    ("Input", util::markdown_code_block("fix", &stake.address), false),
                                    ("Fee", util::markdown_code_block("diff", &format!("- {}", stake.fee)), true),
                                ])
                        })
                    })
            })
            .await
            .unwrap();
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("stake").description("Get stkae by hash").create_option(|option| {
        option
            .name("hash")
            .description("A hash")
            .kind(CommandOptionType::String)
            .min_int_value(0)
            .required(true)
    })
}
