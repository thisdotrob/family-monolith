Add graceful shutdown and configurable port.

Tasks
1. In `server/mod.rs`:
   • Accept `Option<u16>` port param in `run(port: Option<u16>)`.
   • Read `port.unwrap_or(config::DEFAULT_PORT)`.
   • Listen on both IPv4/IPv6: `SocketAddr::from(([0,0,0,0,0,0,0,0], port))`.
   • Spawn Axum server in a `tokio::select!` that also waits on `tokio::signal::ctrl_c()`.
   • Log “Shutting down gracefully” before awaiting server drain.

2. Update `src/bin/dev.rs` to pass `None`.

3. Unit test: start on random port (0), send SIGINT after first response, assert server shuts down without panic.

Commit message: "feat(server): graceful shutdown & configurable port"
