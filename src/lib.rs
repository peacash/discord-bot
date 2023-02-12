pub mod bot;
pub mod commands;
use clap::Parser;
use pea_core::*;
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Args {
    /// Discord bot auth token
    #[clap(short, long)]
    pub token: String,
    /// API Endpoint
    #[clap(long, value_parser, default_value = BIND_API)]
    pub api: String,
    /// Development mode
    #[clap(long, value_parser, default_value_t = false)]
    pub dev: bool,
}
