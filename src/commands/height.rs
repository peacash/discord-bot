use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(_options: &[CommandDataOption]) -> String {
    match get::height(HTTP_API).await {
        Ok(height) => format!(
            r"```
{}
```",
            height
        ),
        Err(err) => err.to_string(),
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("height").description("Latest block height")
}
