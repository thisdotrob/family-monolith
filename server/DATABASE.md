# Database Setup

This document provides instructions for setting up and managing the application's database.

## Database Type

The application uses SQLite as its database.

## Creating the Database

The database file is created automatically when the application starts or when migrations are run. The default location is `blobfishapp.sqlite` in the root of the project.

## Running Migrations

To apply the latest database migrations, run the following command:

```bash
cargo run --bin migrate
```

This command will create the database file if it doesn't exist and then apply all pending migrations from the `migrations` directory.
