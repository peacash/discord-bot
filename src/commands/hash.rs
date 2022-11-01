use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(options: &[CommandDataOption]) -> String {
    let option = options.get(0).expect("Expected int option").resolved.as_ref().expect("Expected int object");
    if let CommandDataOptionValue::Integer(int) = option {
        let int = *int as usize;
        match get::hash(HTTP_API, &int).await {
            Ok(hash) => format!(
                r"```
{}
```",
                hash
            ),
            Err(err) => err.to_string(),
        }
    } else {
        "Please provide a valid int".to_string()
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("hash")
        .description("Search a hash")
        .create_option(|option| option.name("height").description("A positive integer").kind(CommandOptionType::Integer).min_int_value(0).required(true))
}
