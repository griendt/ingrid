mod modules;

use crate::modules::Module;
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

/// Ik ben Ingrid, en ik ben een beetje verlegen. Daarom sta ik standaard in stille modus. Mijn commands zijn:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Toon deze help.
    #[command(aliases = ["h", "?"])]
    Help,
    /// Lees Tolkien voor. Geef optioneel een paragraaf-nummer op.
    #[command()]
    Tolkien(String),
}

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    let (module, input): (&dyn Module, String) = match command {
        Command::Help => (&modules::help::Help {}, "".to_string()),
        Command::Tolkien(input) => (&modules::tolkien::Tolkien {}, input),
    };

    let response = module.handle(input);

    bot.send_message(message.chat.id, response).await?;
    Ok(())
}
