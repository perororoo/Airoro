ALTER TABLE osu_scores RENAME TO osu_scores_old;

CREATE TABLE IF NOT EXISTS osu_scores (
  score_id_local INTEGER PRIMARY KEY,
  score_id_osu INTEGER NOT NULL,
  map_id INTEGER NOT NULL,
  osu_id INTEGER NOT NULL,
  rank_letter TEXT NOT NULL,
  score_lazer INTEGER NOT NULL,
  score_classic INTEGER NOT NULL,
  score_stable INTEGER NOT NULL,
  accuracy REAL NOT NULL,
  count_300 INTEGER NOT NULL,
  count_100 INTEGER NOT NULL,
  count_50 INTEGER NOT NULL,
  count_miss INTEGER NOT NULL,
  pp REAL NOT NULL,
  date_set INTEGER NOT NULL,
  FOREIGN KEY (osu_id)
    REFERENCES osu_profiles (osu_id),
  FOREIGN KEY (rank_letter)
    REFERENCES osu_score_rank (rank_letter)
);


INSERT INTO osu_scores (
  score_id_osu, 
  map_id,
  osu_id,
  rank_letter,
  score_lazer,
  score_classic,
  score_stable,
  accuracy,
  count_300,
  count_100,
  count_50,
  count_miss,
  pp,
  date_set
)
SELECT 
  score_id, 
  map_id,
  osu_id,
  rank_letter,
  score_lazer,
  score_classic,
  score_stable,
  accuracy,
  count_300,
  count_100,
  count_50,
  count_miss,
  pp,
  date_set
FROM osu_scores_old;

DROP TABLE osu_scores_old;