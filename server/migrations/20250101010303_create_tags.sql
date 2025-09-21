-- Create tags table with normalized unique names
CREATE TABLE IF NOT EXISTS tags (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now'))
);

-- Case-insensitive search support
CREATE INDEX IF NOT EXISTS idx_tags_name_lower ON tags(lower(name));

-- updated_at trigger with millisecond precision
CREATE TRIGGER IF NOT EXISTS tags_updated_at
AFTER UPDATE OF name ON tags
FOR EACH ROW
BEGIN
  UPDATE tags SET updated_at = (strftime('%Y-%m-%d %H:%M:%f','now')) WHERE id = NEW.id;
END;
