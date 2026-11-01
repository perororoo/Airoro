ALTER TABLE osu_profile_link RENAME TO osu_profile_link_old;

CREATE TABLE IF NOT EXISTS osu_profile_link (
  link_id INTEGER PRIMARY KEY,
  discord_id INTEGER NOT NULL UNIQUE,
  osu_id INTEGER NOT NULL,
  FOREIGN KEY (discord_id)
    REFERENCES airoros_friends (discord_id),
  FOREIGN KEY (osu_id)
    REFERENCES osu_profiles (osu_id)
);

INSERT INTO osu_profile_link (discord_id, osu_id)
SELECT discord_id, osu_id FROM osu_profile_link_old;

DROP TABLE osu_profile_link_old;