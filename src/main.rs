
use std::sync::Arc;
use std::vec;
use async_openai::types::chat::ChatCompletionRequestSystemMessage;
use serenity::model::gateway::GatewayIntents;
use serenity::secrets::Token;
use serenity::Client;
pub mod utils;
pub mod msg_events;
pub mod msg_commands;
use dotenv::dotenv;
use utils::init_utils::*;
use crate::event_handler::Handler;
use async_openai;
use std::any::Any;
use serenity::model::id::UserId;


#[tokio::main]
async fn main() {
  dotenv().ok(); // Initialize environment variables

  airoro_log!("[MAIN::INIT]: Environment variables loaded successfully !!");
  airoro_log!("[MAIN::INIT]: Getting BOT_USER_ID...");
  let bot_user_id  = UserId::new(dotenv::var("BOT_USER_ID").expect("\n[ERROR]: Cannot get BOT_USER_ID from .env!! Does it not exist?\n").parse::<u64>().unwrap());
  airoro_log!("[MAIN::INIT] BOT_USER_ID Found!!: {}", bot_user_id);
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

  airoro_log!("[MAIN::INIT]: Creating an OpenAI API instance...");
  let openai_client = async_openai::Client::new();
  let mut tokens_used = 0; // initialize token count
  airoro_log!("[MAIN::INIT]: Initializing Airoro Chat Memory...");
  let mut chat_memory = vec![];
  
  let token = Token::from_env("DISCORD_TOKEN").expect("Expected a token in the environment"); // Get bot token from .env
  let mut total_cost = 0.0;

  airoro_log!("[MAIN::LOGIN]: Token loaded successfully !!");

  let intents =  // Set Discord Gateway Intents
    GatewayIntents::GUILD_MESSAGES |
    GatewayIntents::DIRECT_MESSAGES |
    GatewayIntents::MESSAGE_CONTENT;

  let mut bot_client = 
    Client::builder(token, intents) // Logs in the bot !!
    .event_handler(Arc::new(Handler {
      bot_user_id,
      database,
      openai_client,
      chat_memory: chat_memory.into(),
      tokens_used: tokens_used.into(),
      total_cost: total_cost.into(),
    })) // Initialize event handler (see event_handler.rs)
    .await
    .expect("Err creating client"); 

  airoro_log!("[LOGIN]: Bot logged in successfully !!");
  
  // Start listening for events by starting a single shard
  if let Err(why) = bot_client.start().await {
    airoro_log!("[INIT]: Client error: {why:?}");
  }
}