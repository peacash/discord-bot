pub mod bot;
pub mod commands;
pub mod util;
use clap::Parser;
use serenity::utils::Color;
use tofuri_core::*;
pub const EMBED_COLOR: Color = Color::from_rgb(47, 49, 54);
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Discord bot auth token
    #[clap(short, long)]
    pub token: String,
    /// API Endpoint
    #[clap(long, value_parser, default_value = HTTP_API)]
    pub api: String,
    /// Development mode
    #[clap(long, value_parser, default_value_t = false)]
    pub dev: bool,
}
