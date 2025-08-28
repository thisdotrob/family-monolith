# Architecture

This document provides a high-level overview of the project's architecture, outlining the purpose of each module within the `src` directory.

## Root Crate (`src/lib.rs`)

The `lib.rs` file is the entry point of the library crate. It assembles all the other modules into a cohesive application, exporting the primary `run` function from the `server` module to start the application.

## Modules

### `auth`

The `auth` module is responsible for all authentication and authorization logic.

- **JWT**: Handles the creation (`encode`) and validation (`decode`) of JSON Web Tokens.
- **Password**: Provides utilities for password hashing and verification.
- **Guard**: Implements `async-graphql` guards to protect GraphQL endpoints, ensuring that only authenticated users can access certain resources.
- **Refresh**: Manages refresh tokens for persistent login sessions.

### `bin`

This directory contains the executable binaries for the project.

- **`dev.rs`**: The main entry point for running the development server. It initializes the logger, database, and starts the Axum server.

### `config`

The `config` module centralizes application configuration. It defines constants for:

- Database file path (`DB_PATH`)
- JWT secret key
- Frontend origin URL for CORS

### `db`

The `db` module manages all database interactions.

- It uses `sqlx` to create and manage a thread-local SQLite connection pool.
- Provides an `init` function to establish the database connection.
- Offers helper functions (`execute`, `fetch_one`) to simplify common database operations.

### `error` & `error_codes`

These modules define the application's error handling strategy.

- **`error.rs`**: Defines a custom `AppError` struct that can be converted into a structured JSON error response for the client. It maps internal error types to appropriate HTTP status codes.
- **`error_codes.rs`**: Provides an `ErrorCode` enum for standardizing error types across the application.

### `graphql`

This module implements the GraphQL API layer using `async-graphql`.

- It defines the GraphQL schema, including the root query, mutations, and subscriptions (currently empty).
- The `AuthMutation` provides login and token refresh capabilities.
- It integrates with the `db` module by adding the connection pool to the GraphQL context.
- Implements security measures like query depth and complexity limits, and disables introspection in production.

### `server`

The `server` module is responsible for the HTTP server implementation using `axum`.

- **Routes**: Defines all API routes, including `/v1/healthz`, `/v1/version`, and the main `/v1/graphql` endpoint.
- **Middleware**: Implements several middleware layers for:
    - Logging (`TraceLayer`)
    - CORS handling
    - JWT validation
    - Content-Type enforcement (`application/json`)
    - Request body size limits
    - Rate limiting for login attempts
- **Graceful Shutdown**: Handles graceful server shutdown on a `ctrl-c` signal.
