
use std::any::Any;

use async_openai:: { 
  config::OpenAIConfig,
  types::responses::{
    InputItem
  }
};
use serenity::all::{EditInteractionResponse, UserId};
use serenity::async_trait;
use serenity::gateway::client::{ Context, EventHandler };
use serenity::model::event::FullEvent;
use crate::command_registration::check_existing_global_commands;
use crate::database_init;
use crate::{ airoro_log, msg_events::*, msg_commands::* };

#[derive(Clone, Debug)]
pub struct ChatTurn {
    pub role: String, // "user" or "assistant"
    pub content: String,
}
pub struct Handler {
  pub bot_user_id: UserId,
  pub database: sqlx::SqlitePool,
  pub openai_client: async_openai::Client<OpenAIConfig>,
  pub chat_memory: tokio::sync::Mutex<Vec<ChatTurn>>, // Use VecDeque in the future
  pub tokens_used: tokio::sync::Mutex<u32>,
  pub total_cost: tokio::sync::Mutex<f64>,
}

#[async_trait]
impl EventHandler for Handler {
  async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
    match event {
      FullEvent::Message { 
        new_message, .. 
      } => {
        if !new_message.author.bot() {
          let mut tokens_used = self.tokens_used.lock().await;
          let mut chat_memory = self.chat_memory.lock().await;
          let mut total_cost = self.total_cost.lock().await;
          tokio::join!( // remove this if it breaks something
            helo::catch_message(&ctx, &new_message), 
            gacha::mambo::catch_message(&ctx, &new_message),
            gacha::hachimi::catch_message(&ctx, &new_message),
            gacha::chiyono::catch_message(&ctx, &new_message),
            gacha::helios::catch_message(&ctx, &new_message),
            osu::catch_message(&ctx, &new_message),
            five_digit_ph::catch_message(&ctx, &new_message),
            airoro_chat::catch_message(&ctx, &new_message, &self.openai_client, &mut *tokens_used, &mut *chat_memory, &self.bot_user_id, &mut *total_cost),
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
            "team_sheet_search" => team_sheet_search::catch_interaction(command).await,
            "refresh_commands" => refresh_commands::catch_interaction(&ctx).await,
            "register" => register::catch_interaction(command, &self.database).await,
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

