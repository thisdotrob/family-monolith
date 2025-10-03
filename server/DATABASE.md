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
- saved_views
  - id TEXT PRIMARY KEY
  - project_id TEXT NOT NULL (FK projects.id)
  - name TEXT NOT NULL
  - filters TEXT NOT NULL (JSON string)
  - created_by TEXT NOT NULL (FK users.id)
  - created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
  - indices: project_id
  - unique(project_id, LOWER(TRIM(name))) for case-insensitive name uniqueness per project
- tasks
  - id TEXT PRIMARY KEY
  - project_id TEXT NOT NULL (FK projects.id)
  - author_id TEXT NOT NULL (FK users.id)
  - assignee_id TEXT NULL (FK users.id)
  - series_id TEXT NULL (FK recurring_series.id)
  - title TEXT NOT NULL
  - description TEXT NULL
  - status TEXT NOT NULL CHECK (status IN ('todo', 'done', 'abandoned'))
  - scheduled_date TEXT NULL (YYYY-MM-DD format)
  - scheduled_time_minutes INTEGER NULL CHECK (scheduled_time_minutes >= 0 AND scheduled_time_minutes <= 1439)
  - deadline_date TEXT NULL (YYYY-MM-DD format)
  - deadline_time_minutes INTEGER NULL CHECK (deadline_time_minutes >= 0 AND deadline_time_minutes <= 1439)
  - completed_at TEXT NULL
  - completed_by TEXT NULL (FK users.id)
  - abandoned_at TEXT NULL
  - abandoned_by TEXT NULL (FK users.id)
  - created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now'))
  - updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%d %H:%M:%f','now'))
  - indices: project_id, status, author_id, assignee_id, series_id, scheduled_date, deadline_date, updated_at
  - composite indices: (project_id, status)
  - trigger: updates updated_at on modification
- task_tags
  - task_id TEXT NOT NULL (FK tasks.id) ON DELETE CASCADE
  - tag_id TEXT NOT NULL (FK tags.id) ON DELETE CASCADE
  - indices: task_id, tag_id
  - unique(task_id, tag_id)
- project_default_view
  - project_id TEXT PRIMARY KEY (FK projects.id)
  - saved_view_id TEXT NULL (FK saved_views.id)

## Entity Relationships

### Core Entities
- **Users**: Authentication and ownership base entity
- **Projects**: Top-level containers for tasks, owned by users with optional members
- **Tasks**: Work items within projects, can be assigned and have scheduling/deadlines
- **Tags**: Reusable labels that can be attached to tasks and recurring series
- **Recurring Series**: Templates for generating recurring tasks with RRULE patterns

### Relationships
- **Project Ownership**: Each project has an owner (users.id → projects.owner_id)
- **Project Membership**: Users can be members of projects (many-to-many via project_members)
- **Task Authoring**: Tasks are created by users (users.id → tasks.author_id)
- **Task Assignment**: Tasks can be assigned to project members (users.id → tasks.assignee_id)
- **Task Completion**: Tasks track who completed/abandoned them (users.id → tasks.completed_by/abandoned_by)
- **Task-Project**: Tasks belong to projects (projects.id → tasks.project_id)
- **Task-Series**: Tasks can be generated from recurring series (recurring_series.id → tasks.series_id)
- **Task-Tags**: Many-to-many relationship via task_tags junction table
- **Series-Project**: Recurring series belong to projects (projects.id → recurring_series.project_id)
- **Series-Tags**: Many-to-many relationship via recurring_series_tags junction table
- **Saved Views**: Custom filters per project (projects.id → saved_views.project_id)
- **Default Views**: Projects can have a default saved view (saved_views.id → project_default_view.saved_view_id)

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
