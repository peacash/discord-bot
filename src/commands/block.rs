use pea_api::get;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{application_command::ApplicationCommandInteraction, application_command::CommandDataOptionValue, InteractionResponseType},
        prelude::command::CommandOptionType,
        Timestamp,
    },
    prelude::Context,
    utils::Color,
};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(hash) = command.data.options.get(0).expect("Expected address option").resolved.as_ref().expect("Expected address object") {
        let block = match get::block(HTTP_API, hash).await {
            Ok(a) => a,
            Err(_) => get::Block {
                previous_hash: "".to_string(),
                timestamp: 0,
                public_key: "".to_string(),
                signature: "".to_string(),
                transactions: vec![],
                stakes: vec![],
            },
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
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
                                        block.public_key
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
    command
        .name("block")
        .description("Get block by hash")
        .create_option(|option| option.name("hash").description("A hash").kind(CommandOptionType::String).min_int_value(0).required(true))
}
