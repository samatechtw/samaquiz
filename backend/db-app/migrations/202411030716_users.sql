CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $body$
BEGIN
    NEW."updated_at" = now();
    RETURN NEW;
END;
$body$ LANGUAGE 'plpgsql';

CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL DEFAULT '',
    description TEXT NOT NULL DEFAULT '',
    link TEXT NOT NULL DEFAULT '',
    location TEXT NOT NULL DEFAULT '',
    email TEXT UNIQUE,
    password_hash TEXT,
    user_type TEXT NOT NULL,
    user_status TEXT NOT NULL,
    email_confirmed BOOLEAN DEFAULT FALSE NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);

CREATE TRIGGER users_modified_column
BEFORE UPDATE ON users FOR EACH ROW
EXECUTE PROCEDURE update_modified_column()
