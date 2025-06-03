--
-- Drop the old FK constraints on user_id
--

ALTER TABLE quizzes            DROP CONSTRAINT IF EXISTS quizzes_user_id_fkey;
ALTER TABLE quiz_assets        DROP CONSTRAINT IF EXISTS quiz_assets_user_id_fkey;
ALTER TABLE question_assets    DROP CONSTRAINT IF EXISTS question_assets_user_id_fkey;
ALTER TABLE quiz_sessions      DROP CONSTRAINT IF EXISTS quiz_sessions_user_id_fkey;
ALTER TABLE participants       DROP CONSTRAINT IF EXISTS participants_user_id_fkey;

--
-- Re‐add constraints with the correct ON DELETE actions
--

ALTER TABLE quizzes
ADD CONSTRAINT quizzes_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users(id)
  ON DELETE CASCADE;

ALTER TABLE quiz_assets
ADD CONSTRAINT quiz_assets_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users(id)
  ON DELETE CASCADE;

ALTER TABLE question_assets
ADD CONSTRAINT question_assets_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users(id)
  ON DELETE CASCADE;

ALTER TABLE quiz_sessions
ADD CONSTRAINT quiz_sessions_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users(id)
  ON DELETE CASCADE;

-- ON DELETE SET NULL for participants
ALTER TABLE participants
ADD CONSTRAINT participants_user_id_fkey
  FOREIGN KEY (user_id)
  REFERENCES users(id)
  ON DELETE SET NULL;
