use crate::Bot;
use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::Timestamp;
use serenity::prelude::Context;
use serenity::utils::Color;
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
        let block = match get::block(&bot.api, hash).await {
            Ok(a) => a,
            Err(_) => pea_api::Block {
                hash: "".to_string(),
                previous_hash: "".to_string(),
                timestamp: 0,
                address: "".to_string(),
                signature: "".to_string(),
                pi: "".to_string(),
                beta: "".to_string(),
                transactions: vec![],
                stakes: vec![],
            },
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(Color::from_rgb(47, 49, 54))
                                .timestamp(Timestamp::from_unix_timestamp(block.timestamp.into()).unwrap())
                                .fields(vec![
                                    (
                                        "Previous Hash",
                                        format!(
                                            "```ini
[{}]
```",
                                            block.previous_hash
                                        ),
                                        false,
                                    ),
                                    (
                                        "Forger",
                                        format!(
                                            "```fix
{}
```",
                                            block.address
                                        ),
                                        true,
                                    ),
                                    (
                                        "Signature",
                                        format!(
                                            "```json
\"{}\"
```",
                                            block.signature
                                        ),
                                        false,
                                    ),
                                    (
                                        "Transactions",
                                        if block.transactions.is_empty() {
                                            format!(
                                                "```diff
- {}
```",
                                                block.transactions.len()
                                            )
                                        } else {
                                            format!(
                                                "```diff
+ {}
```",
                                                block.transactions.len()
                                            )
                                        },
                                        true,
                                    ),
                                    (
                                        "Stakes",
                                        if block.stakes.is_empty() {
                                            format!(
                                                "```diff
- {}
```",
                                                block.stakes.len()
                                            )
                                        } else {
                                            format!(
                                                "```diff
+ {}
```",
                                                block.stakes.len()
                                            )
                                        },
                                        true,
                                    ),
                                ])
                        })
                    })
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
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
