# Monolith-Backend Development Checklist

> Mark each `[ ]` as `[x]` when done.  
> Follow the numbered order—each step depends on the previous ones.

---

## 1. Bootstrap

- [ ] **Bootstrap-01** – create Cargo project  
  - [ ] `cargo new monolith-backend --lib`  
  - [ ] Update package metadata (`edition`, `description`)  
  - [ ] Init Git repo & commit “chore: initial cargo skeleton”
- [ ] **Bootstrap-02** – linting infrastructure  
  - [ ] Add `[workspace]` table to `Cargo.toml`  
  - [ ] Create `.cargo/config.toml` with Clippy flags  
  - [ ] Add pre-commit hook (fmt + clippy)  
  - [ ] Commit “chore: linting infrastructure”
- [ ] **Bootstrap-03** – add core dependencies  
  - [ ] Insert all runtime deps in `Cargo.toml`  
  - [ ] Commit “chore: add core dependencies”
- [ ] **Bootstrap-04** – scaffold modules  
  - [ ] Create module files/folders (`auth`, `config`, `db`, `error_codes`, `graphql`, `server`)  
  - [ ] Re-export `server::run` from `lib.rs`  
  - [ ] Commit “feat: scaffold top-level modules”

---

## 2. Configuration & Logging

- [ ] **Config-01** – basic constants & test  
  - [ ] Implement `config.rs` (port, DB path, JWT secret, frontend origin)  
  - [ ] Unit test for non-empty constants  
  - [ ] Commit “feat(config): basic configuration constants”
- [ ] **Logging-01** – stdout + file rotation  
  - [ ] Add `server/logging.rs` (`tracing_subscriber` setup)  
  - [ ] Add `tracing-appender` to deps  
  - [ ] Commit “feat(logging): stdout+file logging with hourly rotation”

---

## 3. HTTP Server

- [ ] **Server-01** – health endpoint  
  - [ ] Implement minimal Axum router with `GET /v1/healthz`  
  - [ ] Create `src/bin/dev.rs` and test  
  - [ ] Commit “feat(server): health endpoint with logging”
- [ ] **Server-02** – graceful shutdown & port arg  
  - [ ] Modify `run(port: Option<u16>)`  
  - [ ] Handle `Ctrl-C` → graceful drain  
  - [ ] Unit test shutdown behavior  
  - [ ] Commit “feat(server): graceful shutdown & configurable port”

---

## 4. Database Layer

- [ ] **DB-01** – connection pool  
  - [ ] Implement `db::init` + `db::pool` (OnceCell)  
  - [ ] Attach pool as Axum `Extension`  
  - [ ] Close pool on shutdown & test  
  - [ ] Commit “feat(db): SQLite connection pool wired into app state”
- [ ] **DB-02** – migrations & CLI  
  - [ ] Create `migrations/` SQL files (`users`, `refresh_tokens`)  
  - [ ] Add `sqlx-cli` dev-dependency  
  - [ ] New binary `migrate.rs` running `sqlx::migrate!`  
  - [ ] Commit “feat(db): migrations folder + sqlx migrate CLI”
- [ ] **DB-03** – helper functions  
  - [ ] Add generic `fetch_one` / `execute` in `db/helpers.rs`  
  - [ ] Re-export via `db/mod.rs`  
  - [ ] Unit test helper round-trip  
  - [ ] Commit “feat(db): generic fetch_one/execute helpers”

---

## 5. Error Handling

- [ ] **Error-01** – AppError & codes  
  - [ ] Create `error_codes.rs` enum  
  - [ ] Implement `error.rs` with `IntoResponse`  
  - [ ] Re-export at crate root  
  - [ ] Commit “feat(error): AppError with consistent codes”

---

## 6. Authentication

- [ ] **Auth-01** – password helper  
  - [ ] `auth/password.rs::verify` (plaintext) + tests  
  - [ ] Commit “feat(auth): rudimentary password verification”
- [ ] **Auth-02** – JWT utilities  
  - [ ] `auth/jwt.rs` encode/decode + tests  
  - [ ] Commit “feat(auth): JWT utils”
- [ ] **Auth-03** – refresh-token storage  
  - [ ] `auth/refresh.rs` (create, rotate, delete, random gen)  
  - [ ] Tests for lifecycle  
  - [ ] Commit “feat(auth): refresh-token table helpers”

---

## 7. GraphQL API

- [ ] **GraphQL-01** – empty schema & route  
  - [ ] Build `AppSchema` + mount `/v1/graphql`  
  - [ ] Add `async-graphql-axum` dep and integration test  
  - [ ] Commit “feat(gql): empty schema wired to /v1/graphql”
- [ ] **GraphQL-02** – `auth.login`  
  - [ ] Implement `AuthMutation::login` (+ inputs/outputs)  
  - [ ] Update schema builder & tests  
  - [ ] Commit “feat(gql): auth.login mutation”
- [ ] **GraphQL-03** – `auth.refreshToken`  
  - [ ] Implement rotate logic & payload types  
  - [ ] Tests for success/failure  
  - [ ] Commit “feat(gql): auth.refreshToken”
- [ ] **GraphQL-04** – `auth.logout`  
  - [ ] Delete refresh-token row, test invalidation  
  - [ ] Commit “feat(gql): auth.logout”
- [ ] **GraphQL-05** – auth guard & hide introspection  
  - [ ] Create `AuthGuard` extension  
  - [ ] Disable introspection in schema  
  - [ ] Axum middleware to inject `Claims` from `Authorization` header  
  - [ ] Tests for 403 on anonymous introspection  
  - [ ] Commit “feat(sec): GraphQL auth guard & introspection disabled”

---

## 8. HTTP Security Layers

- [ ] **Security-01** – CORS  
  - [ ] Add `CorsLayer` restricted to frontend origin  
  - [ ] Tests for allowed vs blocked origins  
  - [ ] Commit “feat(sec): strict CORS layer”
- [ ] **Security-02** – body size & Content-Type  
  - [ ] `RequestBodyLimitLayer` 1 MB  
  - [ ] Middleware rejecting non-JSON (`/healthz`, `/version` exempt)  
  - [ ] Tests for 413 & 415 responses  
  - [ ] Commit “feat(sec): body size + content-type enforcement”
- [ ] **Security-03** – login rate limit  
  - [ ] Implement token-bucket per IP for `auth.login` (10/min)  
  - [ ] `Retry-After` header on 429  
  - [ ] Test 11th attempt blocked  
  - [ ] Commit “feat(sec): IP rate-limit on login”
- [ ] **Security-04** – random back-off  
  - [ ] 200–500 ms sleep on auth failures (mocked in tests)  
  - [ ] Commit “feat(sec): add 200–500 ms jitter on auth failures”
- [ ] **Security-05** – query limits  
  - [ ] `.limit_depth(5)` + `.limit_complexity(50)`  
  - [ ] Test oversized query rejected  
  - [ ] Commit “feat(sec): depth(5)+fields(50) limits”

---

## 9. Additional Endpoints

- [ ] **Endpoints-01** – `/v1/version`  
  - [ ] Implement route returning crate version JSON  
  - [ ] Unit test response body  
  - [ ] Commit “feat(api): version endpoint”

---

## 10. Observability

- [ ] **Observability-01** – request tracing  
  - [ ] Add `TraceLayer` with IP, latency, status logging  
  - [ ] Extend logger filter for request IDs  
  - [ ] Manual test steps noted in comments  
  - [ ] Commit “feat(obs): structured request tracing with IP”

---

## 11. Testing

- [ ] **Tests-01** – auth unit tests  
  - [ ] Password, JWT, refresh-token coverage  
  - [ ] Commit “test(auth): unit coverage”
- [ ] **Tests-02** – end-to-end flow  
  - [ ] Start server with in-memory DB  
  - [ ] Verify login → refresh → logout workflow  
  - [ ] Commit “test(e2e): login/refresh/logout integration”

---

## 12. Deployment

- [ ] **Deploy-01** – systemd & HTTPS  
  - [ ] Add `deploy/monolith.service` template  
  - [ ] Write `DEPLOY.md` (RasPi steps, rustls-acme use)  
  - [ ] Implement `server/https.rs` with automatic certs  
  - [ ] Commit “docs(deploy): systemd service & rustls-acme”
- [ ] **Deploy-02** – CI & Docker  
  - [ ] `.github/workflows/ci.yml` (build, test, Docker push)  
  - [ ] `Dockerfile` (two-stage)  
  - [ ] Add README badge & usage instructions  
  - [ ] Commit “ci: GH Actions build, test & publish docker image”

---

### ✅ Done!

When every box is checked the backend should compile, pass all tests, obtain its own TLS cert, and be ready to run on your Raspberry Pi.
