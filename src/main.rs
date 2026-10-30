
use std::sync::Arc;
use serenity::model::gateway::GatewayIntents;
use serenity::secrets::Token;
use serenity::Client;
pub mod utils;
pub mod msg_events;
pub mod msg_commands;
use dotenv::dotenv;
use utils::init_utils::*;
use crate::event_handler::Handler;


#[tokio::main]
async fn main() {
  dotenv().ok(); // Initialize environment variables

  airoro_log!("[MAIN::INIT]: Environment variables loaded successfully !!");
  airoro_log!("[MAIN::INIT] Connecting to database...");
  let database = match database_init::initialize_database().await { 
    Ok(pool) => pool,
    Err(why) => {
      airoro_log!("[MAIN::INIT]: Database Initialization failed !! D: {why:?}");
      return;
    }
  };
  airoro_log!("[MAIN::INIT] Connected to database!!");

  // sqlx::migrate!("./migrations").run(&database).await.expect("Couldn't run database migrations"); // Run sqlx migration script

  let token = Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment"); // Get bot token from .env

  airoro_log!("[MAIN::LOGIN]: Token loaded successfully !!");

  let intents =  // Set Discord Gateway Intents
    GatewayIntents::GUILD_MESSAGES |
    GatewayIntents::DIRECT_MESSAGES |
    GatewayIntents::MESSAGE_CONTENT;

  let mut bot_client = 
    Client::builder(token, intents) // Logs in the bot !!
    .event_handler(Arc::new(Handler {
      database
    })) // Initialize event handler (see event_handler.rs)
    .await
    .expect("Err creating client"); 

  airoro_log!("[LOGIN]: Bot logged in successfully !!");
  
  // Start listening for events by starting a single shard
  if let Err(why) = bot_client.start().await {
    airoro_log!("[INIT]: Client error: {why:?}");
  }
}