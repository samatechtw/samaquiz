CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $body$
BEGIN
    NEW."updated_at" = now();
    RETURN NEW;
END;
$body$ LANGUAGE 'plpgsql';

CREATE TABLE quizzes (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    quiz_type TEXT NOT NULL,
    questions_order TEXT[] NOT NULL DEFAULT '{}',
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER quizzes_modified_column
BEFORE UPDATE ON quizzes FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE questions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id uuid NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    question_type TEXT NOT NULL,
    answers_order TEXT[] NOT NULL DEFAULT '{}',
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER questions_modified_column
BEFORE UPDATE ON questions FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE answers (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id uuid NOT NULL REFERENCES questions(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    points INTEGER NOT NULL DEFAULT 0,
    is_correct BOOLEAN DEFAULT FALSE NOT NULL,
    answers_order TEXT[] NOT NULL DEFAULT '{}',
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER answers_modified_column
BEFORE UPDATE ON answers FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE quiz_assets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id uuid REFERENCES quizzes(id) ON DELETE SET NULL,
    user_id uuid NOT NULL REFERENCES users(id),
    size BIGINT NOT NULL DEFAULT 0,
    content_type TEXT NOT NULL,
    state TEXT NOT NULL,
    upload_expires_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER quiz_assets_modified_column
BEFORE UPDATE ON quiz_assets FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();


CREATE TABLE question_assets (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    question_id uuid REFERENCES questions(id) ON DELETE SET NULL,
    user_id uuid NOT NULL REFERENCES users(id),
    size BIGINT NOT NULL DEFAULT 0,
    content_type TEXT NOT NULL,
    state TEXT NOT NULL,
    upload_expires_at timestamp with time zone NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER question_assets_modified_column
BEFORE UPDATE ON question_assets FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();
