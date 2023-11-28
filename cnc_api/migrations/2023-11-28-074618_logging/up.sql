CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE event_logs (
    event_log_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(255) NOT NULL,
    user_id UUID REFERENCES users(user_id),
    timestamp TIMESTAMP NOT NULL DEFAULT current_timestamp,
    details TEXT
);
