-- Create recurring_series_tags join table for default tags on series
CREATE TABLE IF NOT EXISTS recurring_series_tags (
  series_id TEXT NOT NULL,
  tag_id TEXT NOT NULL,
  FOREIGN KEY(series_id) REFERENCES recurring_series(id) ON DELETE CASCADE,
  FOREIGN KEY(tag_id) REFERENCES tags(id) ON DELETE CASCADE,
  UNIQUE(series_id, tag_id)
);

CREATE INDEX IF NOT EXISTS idx_recurring_series_tags_series_id ON recurring_series_tags(series_id);
CREATE INDEX IF NOT EXISTS idx_recurring_series_tags_tag_id ON recurring_series_tags(tag_id);