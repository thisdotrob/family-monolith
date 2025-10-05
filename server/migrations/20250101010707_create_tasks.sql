-- Create tasks table per spec §5,6,7
CREATE TABLE IF NOT EXISTS tasks (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  author_id TEXT NOT NULL,
  assignee_id TEXT,
  series_id TEXT,
  title TEXT NOT NULL,
  description TEXT,
  status TEXT NOT NULL CHECK (status IN ('todo', 'done', 'abandoned')),
  scheduled_date TEXT, -- YYYY-MM-DD format
  scheduled_time_minutes INTEGER CHECK (scheduled_time_minutes >= 0 AND scheduled_time_minutes <= 1439),
  deadline_date TEXT, -- YYYY-MM-DD format
  deadline_time_minutes INTEGER CHECK (deadline_time_minutes >= 0 AND deadline_time_minutes <= 1439),
  completed_at TEXT,
  completed_by TEXT,
  abandoned_at TEXT,
  abandoned_by TEXT,
  created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now')),
  updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now')),
  FOREIGN KEY(project_id) REFERENCES projects(id),
  FOREIGN KEY(author_id) REFERENCES users(id),
  FOREIGN KEY(assignee_id) REFERENCES users(id),
  FOREIGN KEY(completed_by) REFERENCES users(id),
  FOREIGN KEY(abandoned_by) REFERENCES users(id)
);

-- Indices for common queries
CREATE INDEX IF NOT EXISTS idx_tasks_project_id ON tasks(project_id);
CREATE INDEX IF NOT EXISTS idx_tasks_status ON tasks(status);
CREATE INDEX IF NOT EXISTS idx_tasks_author_id ON tasks(author_id);
CREATE INDEX IF NOT EXISTS idx_tasks_assignee_id ON tasks(assignee_id);
CREATE INDEX IF NOT EXISTS idx_tasks_series_id ON tasks(series_id);

-- Index for ordering by schedule/deadline
CREATE INDEX IF NOT EXISTS idx_tasks_scheduled_date ON tasks(scheduled_date);
CREATE INDEX IF NOT EXISTS idx_tasks_deadline_date ON tasks(deadline_date);

-- Index for concurrency control and general ordering
CREATE INDEX IF NOT EXISTS idx_tasks_updated_at ON tasks(updated_at);

-- Composite index for project + status queries
CREATE INDEX IF NOT EXISTS idx_tasks_project_status ON tasks(project_id, status);

-- updated_at trigger with millisecond precision
CREATE TRIGGER IF NOT EXISTS tasks_updated_at
AFTER UPDATE ON tasks
FOR EACH ROW
BEGIN
  UPDATE tasks SET updated_at = (strftime('%Y-%m-%d %H:%M:%f','now')) WHERE id = NEW.id;
END;