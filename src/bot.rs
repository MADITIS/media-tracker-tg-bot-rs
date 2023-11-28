
use teloxide::{prelude::*, utils::command::BotCommands, types};
use dotenv::dotenv;
use std::{env::{self, VarError}, process};

pub struct BotConfig {
    pub bot_token: String,
}

impl BotConfig {
    pub fn config() -> Result<BotConfig, String> {
        log::info!("Getting ENV Vars");

        if let Err(e) = dotenv() {
            return Err(format!("Unable to load .env file: {e:?}"))
        }

        let bot_token: String = match env::var("BOT_TOKEN")  {
            Ok(token) if !token.is_empty() => token,
            Ok(_) => return Err(format!("BOT_TOKEN Env is empty")),
            Err(error) => return Err(format!("BOT_TOKEN Env is not set: {error:?}")),
        };

        return Ok(BotConfig{bot_token})
    }

    pub async fn start(&self, bot_token: &String) {
        let bot = Bot::new(bot_token);
        log::info!("Starting bot...");

        // let chat_id: [i64; 2] = [-1001190551289, 767750661];
        // for id in chat_id {
        //     bot.send_message(ChatId(id), "Bot Started").send().await;
        // }
        Command::repl(bot, BotConfig::answer).await;
    }

    async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
        match cmd {
            Command::Start => bot.send_message(msg.chat.id, "<b>Yes, I am alive.</b>").reply_to_message_id(msg.id).parse_mode(types::ParseMode::Html).await?,
        };

        Ok(())
    }

}

    #[derive(BotCommands, Clone)]
    #[command(rename_rule = "lowercase", description = "These commands are supported:")]
    pub enum Command {
        #[command(description = "show the status.")]
        Start,
    }