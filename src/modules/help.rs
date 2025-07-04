use teloxide::utils::command::BotCommands;
use crate::Command;
use crate::modules::Module;

pub struct Help {}

impl Module for Help {
    fn handle(&self, _input: String) -> String {
        Command::descriptions().to_string()
    }
}