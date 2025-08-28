# Database Setup

This document provides instructions for setting up and managing the application's database.

## Database Type

The application uses SQLite as its database.

## Creating the Database

The database file is created automatically when the application starts. The default location is `blobfishapp.sqlite` in the root of the project.

## Migrations

Database migrations are applied automatically on server startup. The server connects to SQLite (creating the file if needed) and then runs all pending migrations from the `migrations/` directory. No manual step is required.
