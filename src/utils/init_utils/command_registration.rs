use serenity::gateway::client::Context;
use serenity::model::application::Command;
use crate::time::get_time;

pub async fn check_existing_global_commands (ctx: &Context) -> () {
  let global_commands = Command::get_global_commands(&ctx.http).await;

  match global_commands {
    Ok(value) => {
      todo!()
    }
    Err(err) => {
      println!("{} [COMMAND_REGISTRATION_UTILS]: Couldn't check existing global commands!! D: \n{err:?}", get_time());
    }
  }

  
}

pub async fn set_global_commands (ctx: &Context, global_commands: Vec<Command>) {
  if let Err(err) = ctx.http.create_global_commands(&global_commands).await {
      println!("[COMMAND_REGISTRATION_UTILS]: Failed to set global commands!! D: \n{err:?}");
  }
}

pub async fn delete_global_commands (ctx: &Context) {
  if let Err(err) = Command::set_global_commands(&ctx.http, &[]).await {
    println!("[{}] [COMMAND_REGISTRATION_UTILS]: Failed to clear global commands D: \n{err:?}", get_time());
  }
}
