CREATE TABLE airoros_friends (
  discord_id INTEGER NOT NULL PRIMARY KEY,
  nickname TEXT NOT NULL
);

CREATE TABLE osu_profiles (
  osu_id INTEGER NOT NULL,
  discord_id INTEGER NOT NULL UNIQUE,
  PRIMARY KEY (osu_id, discord_id),
  FOREIGN KEY (discord_id)
    REFERENCES airoros_friends (discord_id)
);

CREATE TABLE osu_scores (
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
    REFERENCES osu_profiles (osu_id)
  FOREIGN KEY (rank_letter)
    REFERENCES osu_score_rank (rank_letter)
);

CREATE TABLE osu_score_rank (
  rank_letter INTEGER NOT NULL, --note: from 0-8 (F, A, B, C, D, S, SH, X, XH)
  is_pass INTEGER NOT NULL --note: from 0-1 (True or False)
);