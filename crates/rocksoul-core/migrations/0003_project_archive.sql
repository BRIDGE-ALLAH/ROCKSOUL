-- NULL means active; a timestamp means archived in the Rocksoul registry.
-- This does not move or mutate the user's workspace directory.
ALTER TABLE projects ADD COLUMN archived_at TEXT;
