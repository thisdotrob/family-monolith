# Project Overview

- Purpose: Monorepo that provides a small full-stack scaffold to ship multiple web apps and a mobile app, backed by a Rust server.
- Top-level apps:
  - Web: React + Vite + Tailwind, served by the Rust server under `/:appId`.
  - Mobile: Expo (React Native) app(s) with local build + self-hosted distribution guidance.
  - Backend: Rust (Axum + async-graphql + sqlx/SQLite) with JWT auth, refresh tokens, rate limiting, and static file serving.
- Languages & frameworks:
  - Rust 2024, Axum, async-graphql, sqlx (SQLite), rustls TLS
  - TypeScript, React 19, Vite 7, Tailwind 4, Apollo Client 3
  - Expo SDK 53, React Native 0.79
- Auth model:
  - GraphQL login returns a short-lived JWT and a refresh token (stored server-side in SQLite).
  - Token rotation via `refreshToken` mutation.
  - A simple Apollo client wrapper in `shared/` manages headers and refresh logic.

# Repository Structure

- `server/` — Rust backend service
  - `src/` — Axum server, GraphQL schema, auth utils, DB helpers
  - `migrations/` — SQL migrations (auto-applied on startup)
  - `deploy/` — systemd units and TLS copy helper
  - Key docs: `ARCHITECTURE.MD`, `AUTH.md`, `DATABASE.md`, `DEPLOY.md`, `STATIC_APPS.md`, `STATIC_DOWNLOADS.md`
- `webapp/` — Vite React app
  - `src/` — App shell, login page, global loading
  - `apps/web/*` — Per-app entry modules (consumed via alias `@apps-web`)
  - Key docs: `DEPLOY.md`, `MULTI_APP.md`
- `mobileapp/` — Expo app
  - `src/` — App shell, login page, global loading
  - `apps/mobile/*` — Per-app modules (consumed via alias `@apps-mobile`)
  - Key docs: `README.md`, `BUILD_LOCAL_DISTRIBUTION.md`
- `shared/` — Reusable TS package for both web and mobile
  - `apollo/` — Apollo client factory
  - `contexts/` — AuthContext (token storage contract)
  - `graphql/` — Queries and mutations
- `apps/` — App modules used by the multi-app setup
  - `apps/web/placeholder` and `apps/mobile/placeholder` (examples)
- Root config
  - `.prettierrc.json`, `.prettierignore`, `tsconfig.base.json`
  - `.husky/pre-commit` with format checks + `cargo fmt --check`
  - Root `package.json` sets up Husky and lint-staged

# Getting Started

- Prerequisites
  - Rust toolchain (see `server/DEPLOY.md` for cross-compile prereqs)
  - Node.js (for webapp/mobileapp)
  - Expo tooling for mobile (`npx expo`)
- Run web (dev)
  - `cd webapp`
  - `VITE_APP_ID=placeholder npm run dev`
- Run mobile (dev)
  - `cd mobileapp`
  - `npx expo start`
- Run server (dev)
  - `cargo run --bin dev` (binds to port 4173)

# Web: Multi-App Build & Serve

- App selection (dev/build)
  - Set `VITE_APP_ID=<appId>` before `npm run dev`/`npm run build`.
  - Vite config sets `base: /<appId>/` and outputs to `dist/<appId>`.
  - `src/appSelection.tsx` loads from `@apps-web/*/index.ts` using `import.meta.glob`.
- Create a new web app
  - Create `apps/web/<newAppId>/HomePage.tsx` and `apps/web/<newAppId>/index.ts` exporting a default component.
  - Build: `VITE_APP_ID=<newAppId> npm run build`
  - Deploy: copy `webapp/dist/<newAppId>` to server under `static/<newAppId>`.
- Serve routes (from server)
  - `/:appId` -> `static/<appId>/index.html`
  - `/:appId/*` -> file under `static/<appId>/<path>` or `static/<appId>/assets/<path>`
- References
  - See `webapp/DEPLOY.md` and `server/STATIC_APPS.md`.

# Mobile: Expo App(s)

- App selection
  - Currently hard-coded to `placeholder` in `mobileapp/src/selectMobileApp.ts`.
  - `mobileapp/app.config.ts` sets `APP_ID` and metadata (name/slug/bundle IDs).
- Local distribution
  - Android: build APK locally via `npm run build:android:placeholder`, host file on server.
  - iOS: prebuild with Expo, open in Xcode, build and sideload.
  - See `mobileapp/BUILD_LOCAL_DISTRIBUTION.md` and `server/STATIC_DOWNLOADS.md`.

# Backend: Rust Server

- Features
  - Axum HTTP server with GraphQL endpoint at `/v1/graphql` and health/version routes.
  - HTTPS via rustls; cert/key paths default to Let's Encrypt locations.
  - Static serving for built web bundles under `static/<appId>` and mobile downloads under `static/downloads/...`.
  - Rate limiting for login attempts (default: 10/min per IP).
  - Request size limits, logging, and content-type enforcement.
- Boot modes
  - `dev` binary on port 4173, `prod` binary on port 443.
- Configuration
  - TLS paths override via env vars: `TLS_CERT_PATH`, `TLS_KEY_PATH`.
  - JWT secret in `server/src/config.rs` (`JWT_SECRET`): CHANGE THIS IN PRODUCTION.
  - SQLite DB path: `./blobfishapp.sqlite`.
- Logging
  - Structured logs to STDOUT and hourly-rotated files under `logs/blobfishapp.log`.
  - Configure via `RUST_LOG`, default `monolith=info,tower_http=info`.

# Database & Migrations

- SQLite database is created on first run at `./blobfishapp.sqlite`.
- Migrations in `server/migrations/` auto-apply on server startup.
- Tables
  - `users(id, username UNIQUE, password, first_name)`
  - `refresh_tokens(id, user_id -> users.id, token UNIQUE)`

# Auth & GraphQL

- Schema
  - Query: `me` returns `{ username, firstName }` (requires JWT).
  - Mutations
    - `login(input: { username, password }) -> { success, token, refreshToken, errors }`
    - `refreshToken(input: { refreshToken }) -> { success, token, refreshToken, errors }` (rotates token)
    - `logout(input: { refreshToken }) -> { success }` (requires valid JWT)
- Client usage (shared package)
  - `shared/apollo/createApolloClient.ts` provides an Apollo client with:
    - Auth header injection from storage
    - Refresh flow on token expiry via `REFRESH_TOKEN_MUTATION`
    - Single-instance client cache
  - Storage adapters
    - Web: `webapp/src/LocalStorage.ts` (localStorage)
    - Mobile: `mobileapp/src/LocalStorage.ts` (AsyncStorage)
  - Contexts
    - `shared/contexts/AuthContext.tsx` supplies `getTokens`, `saveTokens`, `logout`, `isAuthenticating`, `isLoggedIn`.
  - GraphQL docs
    - Queries: `shared/graphql/queries.ts`
    - Mutations: `shared/graphql/mutations.ts` and `shared/graphql/auth.ts`

# Formatting, Linting, and CI

- Prettier configuration in `.prettierrc.json` (singleQuote, trailingComma=all, printWidth=100, etc.).
- Root `lint-staged` runs Prettier on staged files.
- Husky pre-commit hook enforces:
  - `npm --prefix webapp run format:check`
  - `npm --prefix mobileapp run format:check`
  - `cargo fmt --check --manifest-path server/Cargo.toml`
- TypeScript base config (`tsconfig.base.json`)
  - `strict: true`, `noUnusedLocals/Parameters: true`, `noUncheckedSideEffectImports: true`, `jsx: react-jsx`.
- Web Vite aliases
  - `@shared` -> `../shared`, `@apps-web` -> `../apps/web`.

# Conventions & Best Practices

- Keep JWT secret and TLS certs secure and configurable via environment.
- Use parameter bindings in SQLX queries (already used) to avoid SQL injection.
- Adopt per-app IDs consistently across web and mobile. Keep `VITE_APP_ID` in sync with folder names under `apps/web/`.
- Web builds must set `VITE_APP_ID` and deploy to `server/static/<appId>` to be served at `/:appId`.
- Client auth flows
  - For unauthenticated mutations (login, refresh), pass `context: { unauthenticated: true }` from Apollo when needed.
  - Store tokens only via the provided storage adapters through `AuthContext`.
- GraphQL server limits
  - Introspection disabled in production schema; depth and complexity limits enforced.
- Logging & observability
  - Use `RUST_LOG` to increase verbosity during debugging. Collect rotated logs from `logs/` for analysis.
- Rate limiting
  - Login is rate limited per IP; keep this in mind when testing with repeat failures.

# Deployment Overview

- Web app deployment
  - Build per app ID and copy `webapp/dist/<appId>` into `<WorkingDirectory>/static/<appId>`.
  - See `webapp/DEPLOY.md` for rsync scripts to Raspberry Pi.
- Backend deployment
  - See `server/DEPLOY.md` for both Docker and native systemd flows, TLS setup, and cross-compilation with `cargo zigbuild`.
- Mobile distribution
  - Host Android APKs and iOS manifests under `server/static/downloads/...`. See `server/STATIC_DOWNLOADS.md`.

# Notable Files

- `server/src/config.rs` — TLS paths, DB path, JWT secret.
- `server/src/graphql/*` — Query and mutation definitions.
- `server/src/server/*` — Routing, middleware (CORS, logging, body limits, rate limit).
- `shared/apollo/createApolloClient.ts` — Apollo client setup with token refresh.
- `shared/contexts/AuthContext.tsx` — Cross-platform token storage contract.
- `webapp/vite.config.ts` — Multi-app base/outDir and aliases.
- `mobileapp/app.config.ts` — Expo app metadata and `extra.APP_ID`.

# Migration of Legacy Agent Files

- Searched for root-level files to migrate: CLAUDE.md, CLAUDE.local.md, codex.md, .codex/*.md, .cursor/rules/*.mdc, .cursorrules.md, .cursorrules, rules.md, .rules.md, .agent.md, .agent.local.md.
- Result: none found at repository root; no content to migrate.

# Open TODOs / Follow-ups (observed)

- Security hardening
  - Replace `JWT_SECRET` default value before production.
  - Confirm CORS configuration and allowed origins.
- Web auth error handling
  - Verify GraphQL error shapes for token expiry and align client error parsing as needed.
- Multi-app mobile
  - Generalize `mobileapp/src/selectMobileApp.ts` and `app.config.ts` to support more than `placeholder`.

