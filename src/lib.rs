pub mod bot;
pub mod commands;
pub mod util;
use clap::Parser;
use serenity::utils::Color;
pub const EMBED_COLOR: Color = Color::from_rgb(43, 45, 49);
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Discord bot auth token
    #[clap(short, long)]
    pub token: String,

    /// API Endpoint
    #[clap(long, value_parser, default_value = "http://localhost:2022")]
    pub api: String,
}
