use crate::Bot;
use pea_api::get;
use serenity::{
    builder::CreateApplicationCommand,
    model::application::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
    utils::Color,
};
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    let height = match get::height(&bot.http_api).await {
        Ok(a) => a.to_string(),
        Err(_) => "Unknown".to_string(),
    };
    if let Err(why) = command
        .create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
                message.embed(|e| {
                    e.color(Color::from_rgb(47, 49, 54)).title("Height").description(format!(
                        "```fix
{}
```",
                        height
                    ))
                })
            })
        })
        .await
    {
        println!("Cannot respond to slash command: {}", why);
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("height").description("Get current height")
}
