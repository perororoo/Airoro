use serenity::builder::{CreateCommand, EditInteractionResponse};
use serenity::model::application::CommandInteraction;
use serenity::gateway::client::Context;
use crate::utils::init_utils::command_registration::*;
use crate::{airoro_log, msg_commands::*};

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("refresh_commands")
    .description("if command break/something doesn't work try using this first ovo)b");

  return command;
}

pub async fn catch_interaction<'a>(ctx: &Context, command: &CommandInteraction) -> EditInteractionResponse<'a> {
  airoro_log!("[REFRESH_COMMANDS]: Triggered!!");

  let global_commands = vec![
    refresh_commands::get_command_info().await,
    team_sheet_search::get_command_info().await,
    register::get_command_info().await
  ];
  delete_global_commands(&ctx).await;
  set_global_commands(&ctx, global_commands).await;

  airoro_log!("[REFRESH_COMMANDS]: Executed Successfully!!");

  let builder = EditInteractionResponse::new().content("Commands refreshed successfully!!");

  return builder;
}