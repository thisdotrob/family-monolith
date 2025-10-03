# Database Setup

This document provides instructions for setting up and managing the application's database.

## Database Type

The application uses SQLite as its database. SQLite foreign key enforcement is explicitly enabled at startup (PRAGMA foreign_keys = ON).

## Creating the Database

The database file is created automatically when the application starts. The default location is `blobfishapp.sqlite` in the root of the project.

## Migrations

Database migrations are applied automatically on server startup. The server connects to SQLite (creating the file if needed) and then runs all pending migrations from the `migrations/` directory. No manual step is required.

### Schema (current)
- users
  - id TEXT PRIMARY KEY
  - username TEXT UNIQUE NOT NULL
  - password TEXT NOT NULL
  - first_name TEXT
- refresh_tokens
  - id TEXT PRIMARY KEY
  - user_id TEXT NOT NULL (FK users.id)
  - token TEXT UNIQUE NOT NULL
- projects
  - id TEXT PRIMARY KEY
  - name TEXT NOT NULL
  - owner_id TEXT NOT NULL (FK users.id)
  - archived_at DATETIME NULL
  - created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - index: owner_id
- project_members
  - project_id TEXT NOT NULL (FK projects.id)
  - user_id TEXT NOT NULL (FK users.id)
  - created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - indices: project_id, user_id
  - unique(project_id, user_id)
- tags
  - id TEXT PRIMARY KEY
  - name TEXT UNIQUE NOT NULL
  - created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now'))
  - updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now'))
  - index: lower(name)
- recurring_series
  - id TEXT PRIMARY KEY
  - project_id TEXT NOT NULL (FK projects.id)
  - created_by TEXT NOT NULL (FK users.id)
  - title TEXT NOT NULL
  - description TEXT NULL
  - assignee_id TEXT NULL (FK users.id)
  - rrule TEXT NOT NULL
  - dtstart_date TEXT NOT NULL
  - dtstart_time_minutes INTEGER NULL
  - deadline_offset_minutes INTEGER NOT NULL
  - created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - indices: project_id, created_by, assignee_id
- recurring_series_tags
  - series_id TEXT NOT NULL (FK recurring_series.id) ON DELETE CASCADE
  - tag_id TEXT NOT NULL (FK tags.id) ON DELETE CASCADE
  - indices: series_id, tag_id
  - unique(series_id, tag_id)

### Tag normalization rules
- Trim leading/trailing whitespace
- Remove leading '#'
- Collapse internal whitespace to a single space
- Lowercase for case-insensitive uniqueness

### Timestamp behavior
- created_at defaults to the insertion time.
- updated_at defaults to insertion time; application logic should explicitly update this column on write operations that modify a project. (No DB trigger is used for projects; tags use a trigger to update updated_at on name change.)

### Rollbacks
- The migration files are idempotent (CREATE TABLE/INDEX IF NOT EXISTS). To revert changes during development you can either delete the SQLite file (blobfishapp.sqlite) or manually run DROP statements for the affected tables and indices.
