Random back-off on auth failures.

Tasks
1. In `auth/password.rs::verify`, when false, `tokio::time::sleep(Duration::from_millis(rand::thread_rng().gen_range(200..=500))).await`.

2. Ensure unit test mocks sleep with `tokio::time::pause` + `advance`.

Commit message: "feat(sec): add 200-500 ms jitter on auth failures"
