use crate::Command;
use crate::modules::Module;
use sea_orm::DatabaseConnection;
use teloxide::utils::command::BotCommands;

pub struct Help {}

impl Module for Help {
    async fn handle(&self, _input: String, _db: &DatabaseConnection) -> String {
        Command::descriptions().to_string()
    }
}
