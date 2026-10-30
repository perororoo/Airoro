use sqlx::SqlitePool;
use crate::airoro_log;

pub async fn initialize_database () -> Result<SqlitePool, sqlx::Error> {
  // airoro_log!("[DATABASE_INIT]");
  let database_url = std::env::var("DATABASE_URL")
    .expect(format!("[DATABASE_INIT]: DATABASE_URL must be set in the .env file").as_str());

  airoro_log!("[DATABASE_INIT] Database URL found!! Initializing database...");
  let pool = SqlitePool::connect(&database_url).await?;
  airoro_log!("[DATABASE_INIT] Database Initialized!!");
  
  return Ok(pool);
}
