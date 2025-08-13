Implement minimal server:

1. In `server/mod.rs`, add:
   - async `pub async fn run()` setting up Axum router with:
     * `GET /v1/healthz` -> returns `StatusCode::OK`.
   - Call `logging::init()` at the start.

2. Update `lib.rs` to export `server::run`.

3. Add a `src/bin/dev.rs`:

```rust
#[tokio::main]
async fn main() {
    monolith_backend::run().await;
}
```

4. Unit test: spawn the server on port 0, perform GET `/v1/healthz`, assert `200`.

Commit message: "feat(server): health endpoint with logging".
