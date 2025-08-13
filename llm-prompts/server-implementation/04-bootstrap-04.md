Create the baseline module tree:

src/
 ├── auth/mod.rs      // empty
 ├── config.rs        // empty
 ├── db/mod.rs        // empty
 ├── error_codes.rs   // empty
 ├── graphql/mod.rs   // empty
 └── server/mod.rs    // empty

Re-export `server::run` from `lib.rs` (to be implemented later).

Commit message: "feat: scaffold top-level modules".
