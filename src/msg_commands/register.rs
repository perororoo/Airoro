use serenity::all::ResolvedValue;
use serenity::builder::{CreateCommand, CreateCommandOption, EditInteractionResponse};
use serenity::model::application::{CommandOptionType, CommandInteraction};
use serenity::gateway::client::Context;
use crate::time::get_time;
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

  let database_pool = match database_init::initialize_database().await {
    Ok(pool) => pool,
    Err(why) => {
      println!("{} [REGISTER]: Couldn't connect to the database: {why:?}", get_time());
      return;
    }
  };

  let options = command.data.options();
  let Some(subcommand) = options.first() else { return; };

  let ResolvedValue::SubCommand(subcommand_data) = &subcommand.value else { return; };
  let Some(subcommand_option) = subcommand_data.first() else { return; };

  if subcommand.name == "osu" {
    if subcommand_option.name == "osu_profile" {
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
                println!("[{}] [REGISTER]: osu! profile linked successfully!!", get_time());

                let builder = EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));

                if let Err(why) = command.edit_response(&ctx.http, builder).await {
                  println!("RIP I think the bot died - peroroo: {why:?}")
                }
              }
              Err(why) => {
                println!("[{}] [REGISTER]: Something went wrong, couldn't register osu profile TvT {why:?}", get_time());
                let builder = EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));

                if let Err(why) = command.edit_response(&ctx.http, builder).await {
                  println!("RIP I think the bot died - peroroo: {why:?}")
                }
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
                println!("[{}] [REGISTER]: Registered {} successfully!!", get_time(), new_username);

                let database_osu_id = sqlx::query!(
                  "INSERT OR REPLACE INTO osu_profiles (osu_id, discord_id) VALUES (?, ?)",
                  value,
                  new_username
                )
                .execute(&database_pool)
                .await;

                match database_osu_id {
                  Ok(_result) => {
                    println!("[{}] [REGISTER]: osu! profile linked successfully!!", get_time());

                    let builder = EditInteractionResponse::new().content(format!("Registration successful!! \n-# Sorry, this command is not finished yet!"));

                    if let Err(why) = command.edit_response(&ctx.http, builder).await {
                      println!("RIP I think the bot died - peroroo: {why:?}")
                    }
                  }
                  Err(why) => {
                    println!("[{}] [REGISTER]: Something went wrong, couldn't register osu profile TvT {why:?}", get_time());
                    let builder = EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));

                    if let Err(why) = command.edit_response(&ctx.http, builder).await {
                      println!("RIP I think the bot died - peroroo: {why:?}")
                    }
                  }
                }
              }
              Err(why) => { 
                println!("[{}] [REGISTER]: Something went wrong, couldn't register {} TvT {why:?}", get_time(), new_username);
              }
            }
          }
        }
      }
    }
  }
 
  if subcommand.name == "set-nickname" {
    if subcommand_option.name == "nickname" {
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
                println!("[{}] [REGISTER]: Nickname changed successfully!!", get_time());

                let builder = EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", value));

                if let Err(why) = command.edit_response(&ctx.http, builder).await {
                  println!("RIP I think the bot died - peroroo: {why:?}")
                }
              }
              Err(why) => {
                println!("[{}] [REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}", get_time());
                let builder = EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
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
                println!("[{}] [REGISTER]: Nickname changed successfully!!", get_time());
                let builder = EditInteractionResponse::new().content(format!("Nickname set successfully!! Hello {}!! o// \n-# Sorry, this command is not finished yet!", value));

                if let Err(why) = command.edit_response(&ctx.http, builder).await {
                  println!("RIP I think the bot died - peroroo: {why:?}")
                }
              }
              Err(why) => { 
                println!("[{}] [REGISTER]: Something went wrong, couldn't change nickname TvT {why:?}", get_time());
                let builder = EditInteractionResponse::new().content(format!("Sorry, what was it again? An error occured... TvT \n-# Sorry, this command is not finished yet!"));
              }
            }
          }
      }
      }
    }
  }
  database_pool.close().await;
  println!("[{}] [REGISTER]: Executed Successfully!!", get_time());

}