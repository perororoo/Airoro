use sqlx::{SqlitePool};

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct AirorosFriends {
  pub discord_id: i64,
  pub nickname: String
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuProfileLink {
  pub discord_id: i64,
  pub osu_id: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuProfiles {
  pub osu_id: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuScores {
  pub score_id: i64,
  pub map_id: i64,
  pub osu_id: i64,
  pub rank_letter: String,
  pub score_lazer: i64,
  pub score_classic: i64,
  pub score_stable: i64,
  pub accuracy: f32,
  pub count_300: i64,
  pub count_100: i64,
  pub count_50: i64,
  pub count_miss: i64,
  pub pp: f32,
  pub date_set: i64
}

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct OsuScoreRank {
  pub rank_letter: String,
  pub is_pass: i64
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

pub async fn find_osu_id_disc (discord_id: i64, database: &SqlitePool) -> sqlx::Result<OsuProfiles, sqlx::Error> {
  let result = sqlx::query_as::<_, OsuProfiles>(
    "SELECT osu_id FROM osu_profiles WHERE discord_id = ?",
  )
  .bind(discord_id)
  .fetch_one(database)
  .await?;

  Ok(result)
}

pub async fn put_discord_id (discord_id: i64, nickname:&str, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let change_username = sqlx::query!(
    "INSERT INTO 
      airoros_friends (discord_id, nickname) 
      VALUES (?, ?) 
      ON CONFLICT DO 
      UPDATE
      SET
        nickname = excluded.nickname",
    discord_id,
    nickname
  )
  .execute(database)
  .await?;

  Ok(change_username)
}

pub async fn put_osu_id (osu_id: i64, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let database_osu_id = sqlx::query!(
    "INSERT INTO osu_profiles (osu_id)
    VALUES (?)
    ON CONFLICT DO NOTHING",
    osu_id
  )
  .execute(database)
  .await?;

  Ok(database_osu_id)
}

pub async fn link_osu (osu_id: i64, discord_id: i64, database: &SqlitePool) -> sqlx::Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
  let database_osu_id = sqlx::query!(
    "INSERT INTO osu_profile_link (osu_id, discord_id) 
    VALUES (?, ?)
    ON CONFLICT DO 
    UPDATE
    SET
      osu_id = excluded.osu_id",
    osu_id,
    discord_id
  )
  .execute(database)
  .await?;

  Ok(database_osu_id)
}