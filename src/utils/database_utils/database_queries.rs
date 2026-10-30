use sqlx::{SqlitePool};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AirorosFriends {
  pub discord_id: i64,
  pub nickname: String
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuProfileLink {
  discord_id: i64,
  osu_id: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuProfiles {
  osu_id: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuScores {
  score_id: i64,
  map_id: i64,
  osu_id: i64,
  rank_letter: String,
  score_lazer: i64,
  score_classic: i64,
  score_stable: i64,
  accuracy: f32,
  count_300: i64,
  count_100: i64,
  count_50: i64,
  count_miss: i64,
  pp: f32,
  date_set: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuScoreRank {
  rank_letter: String,
  is_pass: i64
}

pub async fn find_dicord_user_id (discord_id: i64, database: &SqlitePool) -> sqlx::Result<AirorosFriends, sqlx::Error> {
  let result = sqlx::query_as::<_, AirorosFriends>(
    "SELECT discord_id FROM airoros_friends WHERE discord_id = ?",
  )
  .bind(discord_id)
  .fetch_one(database)
  .await?;

  Ok(result)
}

pub async fn put_discord_id (discord_id: i64, nickname:&str, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let change_username = sqlx::query!(
    "INSERT OR REPLACE INTO airoros_friends (discord_id, nickname) VALUES (?, ?)",
    discord_id,
    nickname
  )
  .execute(database)
  .await?;

  Ok(change_username)
}

pub async fn put_osu_id (osu_id: i64, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let database_osu_id = sqlx::query!(
    "INSERT OR REPLACE INTO osu_profiles (osu_id) VALUES (?)",
    osu_id
  )
  .execute(database)
  .await?;

  Ok(database_osu_id)
}

pub async fn link_osu (osu_id: i64, discord_id: i64, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let database_osu_id = sqlx::query!(
    "INSERT OR REPLACE INTO osu_profile_link (osu_id, discord_id) VALUES (?, ?)",
    osu_id,
    discord_id
  )
  .execute(database)
  .await?;

  Ok(database_osu_id)
}