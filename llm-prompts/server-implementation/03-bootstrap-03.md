Add runtime dependencies and features:

In `Cargo.toml`, under `[dependencies]`, add:

```toml
axum = { version = "0.7", features = ["macros"] }
async-graphql = "7.0"
sqlx = { version = "0.7", features = ["sqlite", "runtime-tokio-native-tls"] }
jsonwebtoken = "9.3"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "fmt", "json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
rand = "0.8"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "limit", "trace"] }
time = "0.3"
```

Commit message: "chore: add core dependencies".
