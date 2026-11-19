use rosu_v2::Osu;
use rosu_v2::request::GetUserScores;
use sqlx::SqlitePool;
use crate::utils::database_utils::database_queries::find_osu_id_disc;
use crate::airoro_log;

pub async fn get_recent_score<'a>(osu: &'a Osu, database:&SqlitePool, discord_id: i64) -> Option<Vec<rosu_v2::prelude::Score>> {
  let osu_id = find_osu_id_disc(discord_id, database).await;

  match osu_id {
    Ok(osu_profiles) => {
      let recent_score = osu
        .user_scores(osu_profiles.osu_id as u32)
        .recent()
        .include_fails(true)
        .limit(1)
        .await
        .ok()?;

      println!("{}", recent_score.first().unwrap().legacy_score);
      Some(recent_score)
    }
    Err(why) => {
      airoro_log!("[OSU_UTILS::GET_RECENT_SCORE]: Cannot get osu_id from database!!\n{why:?}");
      None
    }
  }
}