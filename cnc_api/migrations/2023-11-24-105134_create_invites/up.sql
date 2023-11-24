CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE invites (
    invite_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    generated_by_user_id UUID NOT NULL,
    has_been_used BOOLEAN NOT NULL,
    date_used TIMESTAMP,
    used_by_user_id UUID,
    invite_code TEXT NOT NULL,
    FOREIGN KEY (generated_by_user_id) REFERENCES users(user_id),
    FOREIGN KEY (used_by_user_id) REFERENCES users(user_id)
);
