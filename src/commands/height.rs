use pea_api::get;
use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    let height = match get::height(HTTP_API).await {
        Ok(a) => a.to_string(),
        Err(_) => "Unknown".to_string(),
    };
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.embed(|e| e.title("Current height").description(height)))
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("height").description("Get current height")
}
