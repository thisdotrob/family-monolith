CREATE TABLE IF NOT EXISTS bathroom_activities (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT NOT NULL,
    timestamp DATETIME NOT NULL,
    consistency TEXT,
    observations TEXT,
    litter_changed BOOLEAN NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);