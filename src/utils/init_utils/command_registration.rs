use serenity::gateway::client::Context;
use serenity::model::application::Command;
use serenity::builder::CreateCommand;
use crate::time::get_time;
use crate::msg_commands::refresh_commands;


pub async fn check_existing_global_commands (ctx: &Context) -> () {
  let global_commands = Command::get_global_commands(&ctx.http).await;

  match global_commands {
    Ok(global_commands_vec) => {
      match global_commands_vec.len() {
        0 => {
          println!("{} [COMMAND_REGISTRATION_UTILS]: There are 0 registered global commands!! Adding \"refresh_commands\"...", get_time());
          set_global_commands(&ctx, vec![refresh_commands::get_command_info().await]).await;
        }

        _ => {
          println!("{} [COMMAND_REGISTRATION_UTILS]: Found {} global commands!! Checking for the \"refresh_commands\" command...", get_time(), global_commands_vec.len());
          let mut count: i8 = 0; 
          for command in &global_commands_vec {
            if command.name == "refresh_commands" {
              println!("{} [COMMAND_REGISTRATION_UTILS]: \"refresh_commands\" command has been found!!", get_time());
              break;
            } else {
              count += 1;
            }
          }
          if (count as usize) == global_commands_vec.len() {
            println!("{} [COMMAND_REGISTRATION_UTILS]: Cannot find \"refresh_commands\" command!! Adding \"refresh_commands\"...", get_time());
            set_global_commands(&ctx, vec![refresh_commands::get_command_info().await]).await;
          }
        }
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