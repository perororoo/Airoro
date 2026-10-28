use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::model::application::{CommandOptionType};

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("refresh_commands")
    .description("if command break/something doesn't work try using this first ovo)b");

  return command;
}