Add `server/logging.rs` with:

```rust
use tracing_subscriber::{fmt, EnvFilter, prelude::*};

pub fn init() {
    let file_appender = tracing_appender::rolling::hourly("logs", "blobfishapp.log");
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive("monolith=info".parse().unwrap()))
        .with(fmt::layer().with_file(true).with_line_number(true))
        .with(fmt::Layer::default().with_writer(file_appender))
        .init();
}
```

Update `Cargo.toml` with `tracing-appender = "0.2"`.

Commit message: "feat(logging): stdout+file logging with hourly rotation".

