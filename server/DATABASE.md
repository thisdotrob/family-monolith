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

### Timestamp behavior
- created_at defaults to the insertion time.
- updated_at defaults to insertion time; application logic should explicitly update this column on write operations that modify a project. (No DB trigger is used to keep schema simple and follow repo style.)

### Rollbacks
- The migration files are idempotent (CREATE TABLE/INDEX IF NOT EXISTS). To revert changes during development you can either delete the SQLite file (blobfishapp.sqlite) or manually run DROP statements for the affected tables and indices.
