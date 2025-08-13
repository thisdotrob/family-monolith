Modify the repository to enable linting on every build:

1. Add the following to the `[workspace]` section in the root `Cargo.toml`:

```toml
[workspace]
members = ["."]
```

2. Create `.cargo/config.toml` with:

```toml
[build]
rustflags = ["-A", "clippy::all"]
```

3. Add a pre-commit Git hook that runs `cargo fmt --all` and `cargo clippy --workspace --all-targets -- -D warnings`.

Commit the new files and updated `Cargo.toml`. Commit message: "chore: linting infrastructure".
