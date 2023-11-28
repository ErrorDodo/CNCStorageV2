-- This file should undo anything in `up.sql`
ALTER TABLE videos
ALTER COLUMN duration TYPE interval
    USING interval '1 second' * duration; -- Convert seconds back to interval
