DROP TABLE IF EXISTS question_assets;

ALTER TABLE
    quizzes
ADD
    COLUMN intro_background_url TEXT NOT NULL DEFAULT '';

ALTER TABLE
    questions
ADD
    COLUMN asset_url TEXT NOT NULL DEFAULT '';

