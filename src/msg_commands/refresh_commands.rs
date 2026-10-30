use serenity::builder::{CreateCommand, EditInteractionResponse};
use serenity::model::application::CommandInteraction;
use serenity::gateway::client::Context;
use crate::command_registration::*;
use crate::msg_commands::*;
use crate::time::get_time;

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("refresh_commands")
    .description("if command break/something doesn't work try using this first ovo)b");

  return command;
}

pub async fn catch_interaction<'a>(ctx: &Context, command: &CommandInteraction) -> EditInteractionResponse<'a> {
  println!("{} [REFRESH_COMMANDS]: Triggered!!", get_time());

  let global_commands = vec![
    refresh_commands::get_command_info().await,
    team_sheet_search::get_command_info().await,
    register::get_command_info().await
  ];
  delete_global_commands(&ctx).await;
  set_global_commands(&ctx, global_commands).await;

  println!("{} [REFRESH_COMMANDS]: Executed Successfully!!", get_time());

  let builder = EditInteractionResponse::new().content("Commands refreshed successfully!!");

  return builder;
}