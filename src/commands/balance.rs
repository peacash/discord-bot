use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(options: &[CommandDataOption]) -> String {
    if let CommandDataOptionValue::String(address) = options.get(0).expect("Expected address option").resolved.as_ref().expect("Expected address object") {
        let balance = match get::balance(HTTP_API, address).await {
            Ok(a) => a.to_string(),
            Err(_) => "Unknown".to_string()
        };
        let balance_staked = match get::balance_staked(HTTP_API, address).await {
            Ok(a) => a.to_string(),
            Err(_) => "Unknown".to_string()
        };
        format!(r"```
Balance: {}
Staked: {}
```", balance, balance_staked)
    } else {
        "Please provide a valid address".to_string()
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("balance")
        .description("Search address balance")
        .create_option(|option| option.name("address").description("An address").kind(CommandOptionType::String).min_int_value(0).required(true))
}
