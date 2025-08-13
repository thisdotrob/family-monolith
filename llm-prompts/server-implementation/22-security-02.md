Body size & content-type limits.

Tasks
1. Add `tower_http::limit::RequestBodyLimitLayer::new(1 * 1024 * 1024)`.

2. Add middleware that rejects requests whose `Content-Type` ≠ `application/json` unless path is `/v1/healthz` or `/v1/version`.

3. Tests: 413 on too-big body; 415 on wrong content type.

Commit message: "feat(sec): body size + content-type enforcement"
