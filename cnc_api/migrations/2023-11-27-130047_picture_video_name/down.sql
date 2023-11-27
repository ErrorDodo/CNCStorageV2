-- This file should undo anything in `up.sql`
ALTER TABLE pictures DROP COLUMN file_name;
ALTER TABLE videos DROP COLUMN file_name;
