use rosu_v2::error::ParsingError;
use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage, EditInteractionResponse};
use serenity::model::application::{CommandOptionType, CommandInteraction};
use serenity::gateway::client::Context;
use crate::time::get_time;

pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("register")
    .description("Become Airoro's friend!! :D")
    .add_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "osu", "Use this to link your osu profile!!")
      .add_sub_option(
        CreateCommandOption::new(
          CommandOptionType::String,
          "osu_profile", 
          "Enter your current osu current username or profile id"
        ).required(true)
      )
    )
    .add_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "set-nickname", "Set your nickname for Airoro to use!!")
      .add_sub_option(
        CreateCommandOption::new(
          CommandOptionType::String,
          "nickname", 
          "What do you want me to call you as? :D"
        ).required(true)
      )
    );

  return command;
}

pub async fn catch_interaction (ctx: &Context, command: &CommandInteraction) {
  println!("{} [REGISTER]: Triggered!!", get_time());

  if let Err(why) = command.defer(&ctx.http).await {
    println!("{} [REGISTER]: Couldn't register user!! D: {why:?}", get_time());
  };

  let options = command.data.options();
  let Some(subcommand) = options.first() else { return; };

  let ResolvedValue::SubCommand(subcommand_data) = &subcommand.value else { return; };
  let Some(subcommand_option) = subcommand_data.first() else { return; };

  if subcommand.name == "osu" {
    if subcommand_option.name == "osu_profile" {
      if let ResolvedValue::String(value) = &subcommand_option.value { 
        let builder = EditInteractionResponse::new().content(format!("[osu] output: {} \n-# Sorry, this command is not finished yet!", value));
        if let Err(why) = command.edit_response(&ctx.http, builder).await {
          println!("RIP I think the bot died - peroroo: {why:?}")
        }
      }
    }
  }
 
  if subcommand.name == "set-nickname" {
    if subcommand_option.name == "nickname" {
      if let ResolvedValue::String(value) = &subcommand_option.value { 
        let builder = EditInteractionResponse::new().content(format!("[set-nickname] output: {} \n-# Sorry, this command is not finished yet!", value));
        if let Err(why) = command.edit_response(&ctx.http, builder).await {
          println!("RIP I think the bot died - peroroo: {why:?}")
        }
      }
    }
  }

  println!("{} [REGISTER]: Executed Successfully!!", get_time());

}