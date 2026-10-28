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
      println!("{} [COMMAND_REGISTRATION::check_existing_global_commands()] Couldn't check existing global commands!! D: \n {err}", get_time());
    }
  }

  
}

pub async fn add_global_commands () {

}

pub async fn delete_global_commands () {

}
