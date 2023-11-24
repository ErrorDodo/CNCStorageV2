CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL,
    password_hash TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    auth_token TEXT NOT NULL,
    date_registered TIMESTAMP NOT NULL,
    invited_by_user_id UUID NOT NULL,
    is_admin BOOLEAN NOT NULL,
    is_moderator BOOLEAN NOT NULL,
    FOREIGN KEY (invited_by_user_id) REFERENCES users(user_id)
);
