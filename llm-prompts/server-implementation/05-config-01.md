Implement initial `config.rs`.

Requirements:
- `pub const DEFAULT_PORT: u16 = 443;`
- `pub const DB_PATH: &str = "blobfishapp.sqlite";`
- `pub const JWT_SECRET: &str = "CHANGE_ME_AT_DEPLOY";`
- Provide `pub fn frontend_origin() -> &'static str { "https://blobfishapp.duckdns.org" }`.

Create a unit test that asserts the constants are not empty.

Commit message: "feat(config): basic configuration constants".
