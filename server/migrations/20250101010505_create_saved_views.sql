CREATE TABLE IF NOT EXISTS saved_views (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  name TEXT NOT NULL,
  filters TEXT NOT NULL, -- JSON string
  created_by TEXT NOT NULL,
  created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  FOREIGN KEY(project_id) REFERENCES projects(id),
  FOREIGN KEY(created_by) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_saved_views_project_id ON saved_views(project_id);

-- Ensure unique name per project (case-insensitive)
CREATE UNIQUE INDEX IF NOT EXISTS idx_saved_views_project_name_unique ON saved_views(project_id, LOWER(TRIM(name)));

CREATE TABLE IF NOT EXISTS project_default_view (
  project_id TEXT PRIMARY KEY,
  saved_view_id TEXT,
  FOREIGN KEY(project_id) REFERENCES projects(id),
  FOREIGN KEY(saved_view_id) REFERENCES saved_views(id)
);