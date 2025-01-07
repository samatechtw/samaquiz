CREATE TABLE quiz_sessions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    quiz_id uuid NOT NULL REFERENCES quizzes(id) ON DELETE CASCADE,
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    code TEXT NOT NULL,
    host_name TEXT NOT NULL,
    host_avatar TEXT,
    start_time BIGINT,
    end_time BIGINT,
    question_end_time BIGINT,
    question_duration BIGINT NOT NULL,
    question_index BIGINT,
    status TEXT NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER quiz_sessions_modified_column
BEFORE UPDATE ON quiz_sessions FOR EACH ROW
EXECUTE PROCEDURE update_modified_column();

CREATE TABLE participants (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id uuid NOT NULL REFERENCES quiz_sessions(id) ON DELETE CASCADE,
    user_id uuid REFERENCES users(id),
    name TEXT NOT NULL,
    avatar TEXT,
    points INTEGER NOT NULL DEFAULT 0,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TABLE responses (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    participant_id uuid NOT NULL REFERENCES participants(id) ON DELETE CASCADE,
    question_id uuid REFERENCES questions(id) ON DELETE CASCADE,
    answer_id uuid REFERENCES answers(id) ON DELETE CASCADE,
    is_correct BOOLEAN,
    created_at timestamp with time zone DEFAULT now() NOT NULL
);
