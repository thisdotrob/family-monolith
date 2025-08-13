| # | Phase | Goal |
| ---- | ---- | ---- |
| 1 | **Project Bootstrap** | Cargo workspace, deps, module stubs, Git repo |
| 2 | **Config + Logging** | Central `config` + tracing-based logging to stdout & file |
| 3 | **Database Layer** | SQLite connection pool, migrations, raw-SQL helpers |
| 4 | **Authentication** | Password hashing placeholder, JWT + refresh-token helpers |
| 5 | **GraphQL Core** | Schema, Axum routes, context injection |
| 6 | **Security Middleware** | CORS, body limits, rate-limit, HTTPS enforcement |
| 7 | **Observability + Health** | `/healthz`, `/version`, structured error codes |
| 8 | **Testing + CI + Deploy** | Unit/integration tests, RasPi systemd service, Let’s Encrypt |
