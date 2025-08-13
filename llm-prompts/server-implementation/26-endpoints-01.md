Add /v1/version endpoint.

Tasks
1. In `server/mod.rs` router: `GET /v1/version` returning JSON `{ "version": env!("CARGO_PKG_VERSION") }`.

2. Unit test.

Commit message: "feat(api): version endpoint"
