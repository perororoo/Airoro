use crate::event_handler::Handler;
use serenity::Client;
use serenity::all::GatewayIntents;
use serenity::secrets::Token;
use std::sync::Arc;
use crate::airoro_log;

pub async fn login_bot() -> Client {
  let token = Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment"); // Get bot token from .env
  
  airoro_log!("[LOGIN]: Token loaded successfully !!");

  let intents =  // Set Discord Gateway Intents
    GatewayIntents::GUILD_MESSAGES |
    GatewayIntents::DIRECT_MESSAGES |
    GatewayIntents::MESSAGE_CONTENT;

  let bot_client = 
    Client::builder(token, intents) // Logs in the bot !!
    .event_handler(Arc::new(Handler)) // Initialize event handler (see event_handler.rs)
    .await
    .expect("Err creating client"); 

  return bot_client;
}
