
use serenity::all::EditInteractionResponse;
use serenity::async_trait;
use serenity::gateway::client::{ Context, EventHandler };
use serenity::model::event::FullEvent;
use crate::command_registration::check_existing_global_commands;
use crate::{ airoro_log, msg_events::*, msg_commands::* };

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
    match event {
      FullEvent::Message { 
        new_message, .. 
      } => {
        if !new_message.author.bot() {
          tokio::join!( // remove this if it breaks something
            helo::catch_message(&ctx, &new_message), 
            gacha::mambo::catch_message(&ctx, &new_message),
            gacha::hachimi::catch_message(&ctx, &new_message),
            gacha::chiyono::catch_message(&ctx, &new_message),
            gacha::helios::catch_message(&ctx, &new_message),
            osu::catch_message(&ctx, &new_message),
            five_digit_ph::catch_message(&ctx, &new_message)
          );
        }
      }

      FullEvent::Ready { 
        data_about_bot, .. 
      } => {
        airoro_log!("[EVENT_HANDLER::READY]: {} is ready !!", data_about_bot.user.name);
        airoro_log!("[EVENT_HANDLER::READY]: Checking existing commands...");
        check_existing_global_commands(&ctx).await;
        airoro_log!("[EVENT_HANDLER::READY]: Checking Successful!!");
      }

      FullEvent::InteractionCreate { 
        interaction, .. 
      } => {
        if let Some(command) = interaction.as_command() {
          if let Err(why) = command.defer(&ctx.http).await {
            airoro_log!("[EVENT_HANDLER::INTERACTION_CREATE]: Couldn't defer message!! D:\n{why:?}");
            return;
          };
          let builder = match command.data.name.as_str() {
            "team_sheet_search" => team_sheet_search::catch_interaction(&ctx, command).await,
            "refresh_commands" => refresh_commands::catch_interaction(&ctx, command).await,
            "register" => register::catch_interaction(&ctx, command).await,
            _ => EditInteractionResponse::new().content("Couldn't find command!!")
          };

          if let Err(why) = command.edit_response(&ctx.http, builder).await {
            airoro_log!("[EVENT_HANDLER::INTERACTION_CREATE]: Cannot respond to slash command!! D:\n{why}");
          }
        }
      }
      _ => {}
    } 
  }
}

