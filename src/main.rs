pub mod utils;
pub mod msg_events;
pub mod msg_commands;

use dotenv::dotenv;
use utils::init_utils::*;

use crate::utils::init_utils::time::get_time;

#[tokio::main]
async fn main() {
  dotenv().ok(); // Initialize environment variables

  println!("[{}] [INIT]: Environment variables loaded successfully !!", get_time());

  let mut bot_client = login::login_bot().await; // Start bot login 

  // Start listening for events by starting a single shard
  if let Err(error) = bot_client.start().await {
    println!("Client error: {error:?}");
  }
}