CREATE TABLE IF NOT EXISTS airoros_friends (
  discord_id INTEGER NOT NULL PRIMARY KEY,
  nickname TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS osu_profile_link (
  discord_id INTEGER NOT NULL PRIMARY KEY,
  osu_id INTEGER NOT NULL,
  FOREIGN KEY (discord_id)
    REFERENCES airoros_friends (discord_id),
  FOREIGN KEY (osu_id)
    REFERENCES osu_profiles (osu_id)
);

CREATE TABLE IF NOT EXISTS osu_profiles (
  osu_id INTEGER NOT NULL PRIMARY KEY
);

CREATE TABLE IF NOT EXISTS osu_scores (
  score_id INTEGER NOT NULL PRIMARY KEY,
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

CREATE TABLE IF NOT EXISTS osu_score_rank (
  rank_letter TEXT NOT NULL PRIMARY KEY, --note: from 0-8 (F, A, B, C, D, S, SH, X, XH)
  is_pass INTEGER NOT NULL --note: from 0-1 (True or False)
);

INSERT INTO osu_score_rank (rank_letter, is_pass)
VALUES
  ('F', 0), -- F, Fail
  ('D', 1), -- D, Pass
  ('C', 1), -- C, Pass
  ('B', 1), -- B, Pass
  ('A', 1), -- A, Pass
  ('S', 1), -- S, Pass
  ('SH', 1), -- SH, Pass
  ('X', 1), -- X, Pass
  ('XH', 1); -- XH, Pass