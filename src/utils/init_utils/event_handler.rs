
use serenity::async_trait;
use serenity::gateway::client::EventHandler;
use serenity::builder::{CreateCommand, CreateCommandOption as CreateOption};
use serenity::model::application::{Command, CommandOptionType};
use serenity::gateway::client::Context;
use serenity::model::event::FullEvent;
use crate::msg_events::*;
use crate::msg_commands::*;
use crate::time::get_time;

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
        println!("[{}] [EVENT_HANDLER]: {} is ready !!", get_time(), data_about_bot.user.name);

        if let Err(why) = Command::set_global_commands(&ctx.http, &[]).await {
          println!("[{}] [EVENT_HANDLER]: Failed to clear global commands: {why:?}", get_time());
        }
        
        

        // let _ = Command::create_global_command(&ctx.http, sheet_command).await;
      }

      FullEvent::InteractionCreate { 
        interaction, .. 
      } => {
        if let Some(interaction) = interaction.as_command() {
          match interaction.data.name.as_str() {
          "team_sheet_search" => { team_sheet_search::catch_interaction(&ctx, interaction).await; }
            _ => {}
          }
        // }
        }
      }
      _ => {}
    } 
  }
}

