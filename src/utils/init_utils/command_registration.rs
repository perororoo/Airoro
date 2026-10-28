use serenity::gateway::client::Context;
use serenity::model::application::Command;
use serenity::builder::CreateCommand;
use crate::time::get_time;
use crate::msg_commands::refresh_commands::get_command_info;


pub async fn check_existing_global_commands (ctx: &Context) -> () {
  let global_commands = Command::get_global_commands(&ctx.http).await;

  match global_commands {
    Ok(global_commands_vec) => {
      if global_commands_vec.is_empty() {
        println!("{} [COMMAND_REGISTRATION_UTILS]: There are 0 registered global commands!! Adding refresh_commands...", get_time());
        set_global_commands(&ctx, vec![get_command_info().await]).await;
      }
    }
    Err(err) => {
      println!("{} [COMMAND_REGISTRATION_UTILS]: Couldn't check existing global commands!! D: \n{err:?}", get_time());
    }
  }
}

pub async fn set_global_commands (ctx: &Context, global_commands: Vec<CreateCommand<'_>>) {
  if let Err(err) = ctx.http.create_global_commands(&global_commands).await {
      println!("[COMMAND_REGISTRATION_UTILS]: Failed to set global commands!! D: \n{err:?}");
  }
}

pub async fn delete_global_commands (ctx: &Context) {
  if let Err(err) = Command::set_global_commands(&ctx.http, &[]).await {
    println!("[{}] [COMMAND_REGISTRATION_UTILS]: Failed to clear global commands D: \n{err:?}", get_time());
  }
}