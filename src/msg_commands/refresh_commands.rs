use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{CommandOptionType};
use serenity::model::application::CommandInteraction;
use serenity::gateway::client::Context;
use crate::command_registration::*;
use crate::msg_commands::*;

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("refresh_commands")
    .description("if command break/something doesn't work try using this first ovo)b");

  return command;
}

pub async fn catch_interaction (ctx: &Context, command: &CommandInteraction) {
  let global_commands = vec![
    refresh_commands::get_command_info().await,
    team_sheet_search::get_command_info().await
  ];

  delete_global_commands(&ctx).await;
  set_global_commands(&ctx, global_commands).await;
}