-- Your SQL goes here
ALTER TABLE videos
ALTER COLUMN duration TYPE bigint
    USING extract(epoch from duration); -- Convert interval to seconds
