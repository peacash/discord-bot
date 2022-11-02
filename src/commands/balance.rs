use crate::Bot;
use pea_api::get;
use pea_core::constants::DECIMAL_PRECISION;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{application_command::ApplicationCommandInteraction, application_command::CommandDataOptionValue, InteractionResponseType},
        prelude::command::CommandOptionType,
    },
    prelude::Context,
    utils::Color,
};
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(address) = command.data.options.get(0).expect("Expected address option").resolved.as_ref().expect("Expected address object") {
        let balance = match get::balance(&bot.http_api, address).await {
            Ok(a) => (a as f64 / DECIMAL_PRECISION as f64).to_string(),
            Err(_) => "Unknown".to_string(),
        };
        let balance_staked = match get::balance_staked(&bot.http_api, address).await {
            Ok(a) => (a as f64 / DECIMAL_PRECISION as f64).to_string(),
            Err(_) => "Unknown".to_string(),
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
                    message.embed(|e| {
                        e.color(Color::from_rgb(47, 49, 54)).fields(vec![
                            (
                                "Address",
                                format!(
                                    "```fix
{}
```",
                                    address
                                ),
                                false,
                            ),
                            (
                                "Balance",
                                format!(
                                    "```diff
+ {}
```",
                                    balance
                                ),
                                true,
                            ),
                            (
                                "Staked",
                                format!(
                                    "```diff
- {}
```",
                                    balance_staked
                                ),
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
        .name("balance")
        .description("Get balance of address")
        .create_option(|option| option.name("address").description("An address").kind(CommandOptionType::String).min_int_value(0).required(true))
}
