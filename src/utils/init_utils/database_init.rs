use sqlx::SqlitePool;
use crate::time::get_time;

pub async fn initialize_database () -> Result<SqlitePool, sqlx::Error> {
  // println!("[{}] [DATABASE_INIT]", get_time());
  let database_url = std::env::var("DATABASE_URL")
    .expect(format!("[{}] [DATABASE_INIT]: DATABASE_URL must be set in the .env file", get_time()).as_str());

  println!("[{}] [DATABASE_INIT] Database URL found!! Initializing database...", get_time());
  let pool = SqlitePool::connect(&database_url).await?;
  println!("[{}] [DATABASE_INIT] Database Initialized!!", get_time());
  
  return Ok(pool);
}
