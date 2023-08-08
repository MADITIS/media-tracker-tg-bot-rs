// use teloxide::{prelude::*, utils::command::BotCommands};
// use dotenv::dotenv;
mod bot;
use std::{env::{self, VarError}, process};

#[tokio::main]
async fn main() {
    env_logger::builder()
    .filter_level(log::LevelFilter::Debug)
    .init();

    let bot = bot::BotConfig::config().unwrap_or_else(|error| {
            log::error!("{error}");
            process::exit(1);
    });

    log::info!("Configured Bot Token");

    bot.start(&bot.bot_token).await;
}


