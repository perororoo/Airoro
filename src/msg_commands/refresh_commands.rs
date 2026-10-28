use serenity::builder::{CreateCommand};
use serenity::gateway::client::Context;
use serenity::model::application::CommandInteraction;

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("refresh_commands")
    .description("if command break/something doesn't work try using this first ovo)b");

  return command;
}

pub async fn catch_interaction (ctx: &Context, command: &CommandInteraction) {

}