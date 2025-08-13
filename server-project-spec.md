# Monolith Backend Comprehensive Specification

## 1. Overview
- **Project Goal:** Build a Rust-based backend for a private family-use monolith application.
- **Architecture:** Monolithic server providing a GraphQL API.
- **Deployment:** Hosted on a Raspberry Pi, exposed over LAN and public internet via HTTPS.
- **Frontend Access:** Public IP only.

---

## 2. Tech Stack
- **Programming Language:** Rust
- **Web Framework:** Axum
- **GraphQL:** async-graphql
- **Database:** SQLite
- **Database Access:** Manual raw SQL queries (no ORM)
- **JWT Management:** HMAC (HS256)
- **SSL/TLS:** Native Let's Encrypt integration

---

## 3. Project Structure
- **Modules:**
  - `auth`: Authentication logic
  - `graphql`: Schema definitions and GraphQL operations
  - `db`: Raw SQL interactions
  - `config`: Centralized configuration constants
  - `server`: HTTP server configuration and routing
  - `error_codes`: Defined error codes for consistency
- **Startup:**
  - Self-check: Database and SSL certificate validation
  - Detailed logging for each startup check
- **Shutdown:**
  - Graceful shutdown on interrupts (closing DB connections)

---

## 4. Authentication System
- **Method:** Username + password (plaintext storage for now)
- **Account Management:** Admin manually pre-creates accounts
- **JWTs:**
  - Signed with a hardcoded HMAC secret
  - Only include the username claim
  - Lifetime: 24 hours
- **Refresh Tokens:**
  - Opaque, secure random strings
  - Rotating tokens on each refresh
  - Stored in SQLite
  - No expiration unless manually revoked
- **Multi-device Support:** Multiple active sessions allowed
- **Logout:** Explicit endpoint to revoke a refresh token

---

## 5. Security Measures
- **CORS:** Restrict to allowed frontend domain
- **HTTPS Only:** No HTTP support
- **Strict Content-Type:** `application/json` only
- **Hide GraphQL Schema:** Require JWT for introspection queries
- **Rate Limits:**
  - Login attempts: 10/minute/IP
  - Includes `Retry-After` headers
- **Random Backoff:** 200–500ms delay on all authentication failures
- **Query Complexity Protection:**
  - Max depth: 5
  - Max fields: 50
- **Body Size Limit:** 1MB max
- **Request Timeout:** 30 seconds
- **Reject WebSocket Upgrade Requests**
- **Handle Unknown Routes:** Return JSON 404 errors

---

## 6. Database Schema
- **Users Table:**
  - `id` (UUID)
  - `username` (lowercased, max 32 characters)
  - `password` (plaintext, max 128 characters)
  - `first_name`
- **Refresh Tokens Table:**
  - `id` (UUID)
  - `user_id` (foreign key)
  - `token` (secure random string)

---

## 7. GraphQL API Design
- **Versioned Endpoint:** `/v1/graphql`
- **Namespacing:**
  - `auth.login(username, password)`
  - `auth.refreshToken(refreshToken)`
  - `auth.logout(refreshToken)`
  - `auth.me`
- **Mutation Response Structure:**
  - `success: true/false`
  - `errors: [{ code, message }]`
  - Optional data (token, refreshToken, etc.)
- **Error Handling:**
  - Custom error codes in `error_codes.rs`
  - Uniform JSON error format

---

## 8. HTTP Endpoints
- `/v1/healthz`: Health check (returns 200 OK)
- `/v1/version`: Returns backend version (e.g., `{ "version": "1.0.0" }`)

---

## 9. Logging Strategy
- **Levels:** INFO, WARN, ERROR
- **Console Logs:** Colorful and pretty
- **File Logs:** Structured plain text with size-based rotation (10MB per file)
- **Immediate Flush:** After every write
- **Timestamps:** ISO 8601 UTC
- **Client IP Logging:** Included with every request
- **Debug Mode:**
  - Logs incoming GraphQL queries and responses
  - Logs errors separately at WARN/ERROR

---

## 10. Network Behavior
- **Port:** Configurable through `config` module
- **IPv4 and IPv6:** Supported
- **Connection Backlog:** OS default

---

## 11. Testing Plan
- **Unit Tests:**
  - Validate GraphQL resolver behaviors
  - Test auth functions (login, logout, refresh)
  - Test token generation and validation
- **Integration Tests:**
  - Full login/refresh/logout flow
  - GraphQL query protection (unauthenticated access)
  - Health and version endpoint responses
- **Manual Testing:**
  - Create sample users in SQLite
  - Access API via localhost and public IP
  - Verify CORS behavior with frontend simulations
  - Test invalid GraphQL queries for proper error responses
- **Security Testing:**
  - Attempt excessive login attempts (rate limit enforced)
  - Attempt schema introspection without auth
  - Verify HTTPS-only access

---

# End of Comprehensive Specification

Ready for immediate developer handoff!
