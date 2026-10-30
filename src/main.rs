pub mod utils;
pub mod msg_events;
pub mod msg_commands;
use dotenv::dotenv;
use utils::init_utils::*;

#[tokio::main]
async fn main() {
  dotenv().ok(); // Initialize environment variables
  
  let database_pool = match database_init::initialize_database().await { 
    Ok(pool) => pool,
    Err(why) => {
      airoro_log!("[INIT]: Database Initialization failed !! D: {why:?}");
      return;
    }
  };

  airoro_log!("[INIT]: Environment variables loaded successfully !!");

  let mut bot_client = login::login_bot().await; // Start bot login 
  
  // Start listening for events by starting a single shard
  if let Err(why) = bot_client.start().await {
    airoro_log!("[INIT]: Client error: {why:?}");
  }
}