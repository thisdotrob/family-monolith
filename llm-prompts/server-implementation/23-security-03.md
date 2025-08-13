Login rate limiting.

Tasks
1. Use `tower::limit::ConcurrencyLimitLayer` is wrong; instead implement IP-based token-bucket:
   - Memory map (DashMap) keyed by IpAddr counting attempts in sliding 60-sec window.
   - Reject with `429 Too Many Requests` & `Retry-After: 60` after 10.

2. Apply only to `/v1/graphql` when `auth.login` is detected (check GraphQL operation name).

3. Tests: 10 successes, 11th returns 429.

4. Run `cargo fmt` after making changes to autoformat the code.

5. Run `cargo build` to make sure the changes compile.

6. Run all the tests with `cargo test` to make sure they all still pass, including the new tests added
with the changes.

Commit message: "feat(sec): IP rate-limit on login"
