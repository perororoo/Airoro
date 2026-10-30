use std::collections::linked_list;

use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::model::application::{CommandOptionType, CommandInteraction};
use sqlx::SqlitePool;
use crate::airoro_log;
use crate::utils::database_utils::database_queries::{find_dicord_user_id, link_osu, put_discord_id, put_osu_id};


pub async fn get_command_info<'a>() -> CreateCommand<'a> {
  let command = CreateCommand::new("register")
    .description("Become Airoro's friend!! :D")
    .add_option(
      CreateCommandOption::new(CommandOptionType::SubCommand, "osu", "Use this to link your osu profile!!")
      .add_sub_option(
        CreateCommandOption::new(
          CommandOptionType::Integer,
          "osu_profile", 
          "Enter your current osu profile id"
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
        )
        .max_length(500)
        .required(true)
      )
    );

  return command;
}

pub async fn catch_interaction<'a>(command: &CommandInteraction, database_pool: &SqlitePool) -> EditInteractionResponse<'a> {
  airoro_log!("[REGISTER]: Triggered!!");

  let options = command.data.options();
  let Some(subcommand) = options.first() else { 
    return EditInteractionResponse::new().content("An error occured!! D:");
  };

  let ResolvedValue::SubCommand(subcommand_data) = &subcommand.value else { 
    return EditInteractionResponse::new().content("An error occured!! D:");
  };
  let Some(subcommand_option) = subcommand_data.first() else { 
    return EditInteractionResponse::new().content("An error occured!! D:");
  };
  
  match (subcommand.name, subcommand_option.name) {
    ("osu", "osu_profile") => {
      if let ResolvedValue::Integer(osu_id) = subcommand_option.value {
        let database_user_id = find_dicord_user_id(command.user.id.get() as i64, database_pool).await;

        match database_user_id {
          Ok(user) => {
            let database_osu_id = put_osu_id(osu_id, database_pool).await;
            let link_osu_id = link_osu(osu_id, command.user.id.get() as i64, database_pool).await;

            match (database_osu_id, link_osu_id) {
              (Ok(_result1), Ok(_result2)) => {
                airoro_log!("[REGISTER]: Executed Successfully!!");
                return EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));
              }
              _ => { // aw hell nah im not writing 3 more cases
                airoro_log!("[REGISTER]: Something went wrong, couldn't register osu profile TvT");
                return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
              }
            }
          }
          Err(_why) => {
            let new_username = command.user.name.as_str();

            let change_username = put_discord_id(command.user.id.get() as i64, new_username, database_pool).await;

            match change_username {
              Ok(_result) => {
                airoro_log!("[REGISTER]: Registered {} successfully!!", new_username);

                let database_osu_id = put_osu_id(osu_id, database_pool).await;
                let link_osu_id = link_osu(osu_id, command.user.id.get() as i64, database_pool).await;

                match (database_osu_id, link_osu_id) {
                  (Ok(_result1), Ok(_result2)) => {
                    airoro_log!("[REGISTER]: Executed Successfully!!");
                    return EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));
                  }
                  _ => { // aw hell nah im not writing 3 more cases
                    airoro_log!("[REGISTER]: Something went wrong, couldn't register osu profile TvT");
                    return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
                  }
                }
              }
              Err(why) => { 
                airoro_log!("[REGISTER]: Something went wrong, couldn't register {} TvT {why:?}", new_username);
                return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
              }
            } 
          }
        }
      } else { return EditInteractionResponse::new().content("An error occured!! D: - Couldn't read your input!!"); }

      
    }

    ("set-nickname", "nickname") => {
      if let ResolvedValue::String(new_username) = subcommand_option.value {

        let database_user_id = find_dicord_user_id(command.user.id.get() as i64, database_pool).await;

        match database_user_id {
          Ok(user_id) => {
            let change_username = put_discord_id(user_id.discord_id, new_username, database_pool).await;
          
            match change_username {
              Ok(_result) => {
                airoro_log!("[REGISTER]: Nickname changed successfully!!");
                return EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", new_username));
              }
              Err(why) => {
                airoro_log!("[REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}");
                return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
              }
            }
          }
          Err(_why) => {
            let change_username = put_discord_id(command.user.id.get() as i64, new_username, database_pool).await;

            match change_username {
              Ok(_result) => {
                airoro_log!("[REGISTER]: Nickname changed successfully!!");
                return EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", new_username));
              }
              Err(why) => { 
                airoro_log!("[REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}");
                return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
              }
            }
          }
        }
      } else { return EditInteractionResponse::new().content("An error occured!! D: - Couldn't read your input!!"); }
    }
    _ => { return EditInteractionResponse::new().content("An error occured!! D: - Couldn't find the subcommand!!"); }
  }

  
}
