use pea_api::get;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{application_command::ApplicationCommandInteraction, application_command::CommandDataOptionValue, InteractionResponseType},
        prelude::command::CommandOptionType,
    },
    prelude::Context,
};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let option = command.data.options.get(0).expect("Expected int option").resolved.as_ref().expect("Expected int object");
    if let CommandDataOptionValue::Integer(height) = option {
        let hash = match get::hash(HTTP_API, &(*height as usize)).await {
            Ok(hash) => format!(
                r"```
{}
```",
                hash
            ),
            Err(err) => err.to_string(),
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.embed(|e| e.title(format!("Hash {}", height)).description(hash)))
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("hash")
        .description("Get hash by height")
        .create_option(|option| option.name("height").description("A positive integer").kind(CommandOptionType::Integer).min_int_value(0).required(true))
}
