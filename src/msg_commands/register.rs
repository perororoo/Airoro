use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::model::application::{CommandOptionType, CommandInteraction};
use serenity::gateway::client::Context;
use crate::airoro_log;
use crate::utils::init_utils::database_init;


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

pub async fn catch_interaction<'a>(ctx: &Context, command: &CommandInteraction) -> EditInteractionResponse<'a> {
  airoro_log!("[REGISTER]: Triggered!!");

  let database_pool = match database_init::initialize_database().await {
    Ok(pool) => pool,
    Err(why) => {
      return EditInteractionResponse::new().content("An error occured!! D:");
    }
  };

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

  if subcommand.name == "osu" && subcommand_option.name == "osu_profile" {
    if let ResolvedValue::Integer(value) = subcommand_option.value { 
      let database_user_id = sqlx::query!(
        "SELECT discord_id FROM airoros_friends WHERE discord_id = ?",
        command.user.id.get() as i64
      )
      .fetch_one(&database_pool)
      .await;

      match database_user_id {
        Ok(user) => {
          let database_osu_id = sqlx::query!(
            "INSERT OR REPLACE INTO osu_profiles (osu_id, discord_id) VALUES (?, ?)",
            value,
            user.discord_id
          )
          .execute(&database_pool)
          .await;

          match database_osu_id {
            Ok(_result) => {
              airoro_log!("[REGISTER]: Executed Successfully!!");
              return EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));
            }
            Err(why) => {
              airoro_log!("[REGISTER]: Something went wrong, couldn't register osu profile TvT {why:?}");
              return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
            }
          }
        }
        Err(_why) => {
          let new_username = command.user.name.as_str();

          let change_username = sqlx::query!(
            "INSERT OR REPLACE INTO airoros_friends (discord_id, nickname) VALUES (?, ?)",
            command.user.id.get() as i64,
            new_username
          )
          .execute(&database_pool)
          .await;

          match change_username {
            Ok(_result) => {
              airoro_log!("[REGISTER]: Registered {} successfully!!", new_username);

              let database_osu_id = sqlx::query!(
                "INSERT OR REPLACE INTO osu_profiles (osu_id, discord_id) VALUES (?, ?)",
                value,
                new_username
              )
              .execute(&database_pool)
              .await;

              match database_osu_id {
                Ok(_result) => {
                  airoro_log!("[REGISTER]: osu! profile linked successfully!!");
                  return EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));
                }
                Err(why) => {
                  airoro_log!("[REGISTER]: Something went wrong, couldn't register osu profile TvT {why:?}");
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
  } else if subcommand.name == "set-nickname" && subcommand_option.name == "nickname" {
    if let ResolvedValue::String(value) = subcommand_option.value {
      let database_user_id = sqlx::query!(
        "SELECT discord_id FROM airoros_friends WHERE discord_id = ?",
        command.user.id.get() as i64
      )
      .fetch_one(&database_pool)
      .await;

      match database_user_id {
        Ok(user_id) => {
          let new_username = value;

          let change_username = sqlx::query!(
            "INSERT OR REPLACE INTO airoros_friends (discord_id, nickname) VALUES (?, ?)",
            &user_id.discord_id,
            new_username
          )
          .execute(&database_pool)
          .await;

          match change_username {
            Ok(_result) => {
              airoro_log!("[REGISTER]: Nickname changed successfully!!");
              return EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", value));
            }
            Err(why) => {
              airoro_log!("[REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}");
              return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
            }
          }
        }
        Err(_why) => {
          let new_username = value;

          let change_username = sqlx::query!(
            "INSERT OR REPLACE INTO airoros_friends (discord_id, nickname) VALUES (?, ?)",
            command.user.id.get() as i64,
            new_username
          )
          .execute(&database_pool)
          .await;

          match change_username {
            Ok(_result) => {
              airoro_log!("[REGISTER]: Nickname changed successfully!!");
              return EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", value));
            }
            Err(why) => { 
              airoro_log!("[REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}");
              return EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
            }
          }
        }
      }
    } else { return EditInteractionResponse::new().content("An error occured!! D: - Couldn't read your input!!"); }
  } else { return EditInteractionResponse::new().content("An error occured!! D: - Couldn't find the subcommand!!"); }
  
}
 
// airoro_log!("[REGISTER]: osu! profile linked successfully!! closing the database...");
// database_pool.close().await;
  
