mod entity;
mod modules;

use crate::entity::chat;
use crate::modules::Module;
use log::{info, warn};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    info!("Starting Ingrid...");
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

impl Command {
    async fn handle(&self, db: &DatabaseConnection, chat: &chat::ActiveModel) -> String {
        match self {
            Command::Help => modules::help::Help {}.handle("".to_string(), db).await,
            Command::Tolkien(input) => {
                modules::tolkien::Tolkien {
                    chat_id: chat.id.clone().unwrap(),
                }
                .handle(input.to_owned(), db)
                .await
            }
        }
    }
}

async fn answer(bot: Bot, message: Message, command: Command) -> ResponseResult<()> {
    warn!(
        "Received from chat ID {} user id {} message: '{}'",
        message.chat.id,
        message
            .from
            .clone()
            .and_then(|user| Some(user.id.to_string()))
            .unwrap_or_default(),
        message.text().unwrap_or_default(),
    );

    let db = Database::connect(ConnectOptions::new(
        dotenv::var("DATABASE_URL").expect("DATABASE_URL environment variable not set."),
    ))
    .await
    .expect("Could not connect to the database.");

    let chat = message.get_or_create_chat(&db).await;
    let response = command.handle(&db, &chat).await;

    bot.send_message(message.chat.id, response).await?;
    Ok(())
}

trait InteractWithDb {
    async fn get_or_create_chat(&self, db: &DatabaseConnection) -> chat::ActiveModel;
}

impl InteractWithDb for Message {
    async fn get_or_create_chat(&self, db: &DatabaseConnection) -> chat::ActiveModel {
        match chat::Entity::find()
            .filter(chat::Column::TelegramId.eq(self.chat.id.0))
            .one(db)
            .await
            .expect("Could not query the database")
        {
            Some(chat) => chat.into_active_model(),
            None => chat::ActiveModel {
                telegram_id: Set(self.chat.id.0 as i32),
                ..Default::default()
            }
            .insert(db)
            .await
            .expect("Could not insert chat")
            .into(),
        }
    }
}
