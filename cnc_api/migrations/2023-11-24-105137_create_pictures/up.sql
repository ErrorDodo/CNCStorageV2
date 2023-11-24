CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE pictures (
    picture_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    uploaded_by_user_id UUID NOT NULL,
    upload_date TIMESTAMP NOT NULL,
    file_url TEXT NOT NULL,
    file_size BIGINT NOT NULL,
    file_format TEXT NOT NULL,
    resolution TEXT NOT NULL,
    tags TEXT[],
    FOREIGN KEY (uploaded_by_user_id) REFERENCES users(user_id)
);
