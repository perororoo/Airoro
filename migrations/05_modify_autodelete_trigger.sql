DROP TRIGGER IF EXISTS auto_delete_osu_id;

CREATE TRIGGER IF NOT EXISTS auto_delete_osu_id
  AFTER UPDATE ON osu_profile_link
BEGIN
  DELETE FROM osu_profiles
  WHERE NOT EXISTS (
    SELECT 1
    FROM osu_profile_link
    WHERE osu_profile_link.osu_id = osu_profiles.osu_id
  )
  AND NOT EXISTS (
    SELECT 1
    FROM osu_scores
    WHERE osu_scores.osu_id = osu_profiles.osu_id
  );
END;