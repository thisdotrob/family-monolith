-- Create recurring_series table to store series templates and rules
CREATE TABLE IF NOT EXISTS recurring_series (
  id TEXT PRIMARY KEY,
  project_id TEXT NOT NULL,
  created_by TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT,
  assignee_id TEXT,
  rrule TEXT NOT NULL,
  dtstart_date TEXT NOT NULL,
  dtstart_time_minutes INTEGER,
  deadline_offset_minutes INTEGER NOT NULL,
  created_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  updated_at DATETIME NOT NULL DEFAULT (CURRENT_TIMESTAMP),
  FOREIGN KEY(project_id) REFERENCES projects(id),
  FOREIGN KEY(created_by) REFERENCES users(id),
  FOREIGN KEY(assignee_id) REFERENCES users(id)
);

CREATE INDEX IF NOT EXISTS idx_recurring_series_project_id ON recurring_series(project_id);
CREATE INDEX IF NOT EXISTS idx_recurring_series_created_by ON recurring_series(created_by);
CREATE INDEX IF NOT EXISTS idx_recurring_series_assignee_id ON recurring_series(assignee_id);