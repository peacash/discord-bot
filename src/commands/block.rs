use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(options: &[CommandDataOption]) -> String {
    if let CommandDataOptionValue::String(hash) = options
        .get(0)
        .expect("Expected hash option")
        .resolved
        .as_ref()
        .expect("Expected hash object")
    {
        match get::block(HTTP_API, &hash).await {
            Ok(block) => format!(
                r"```
{:?}
```",
                block
            ),
            Err(err) => err.to_string(),
        }
    } else {
        "Please provide a valid block hash".to_string()
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("block")
        .description("Search a block")
        .create_option(|option| {
            option
                .name("hash")
                .description("A block hash")
                .kind(CommandOptionType::String)
                .min_int_value(0)
                .required(true)
        })
}
