# Deployment

This document outlines deployment for the backend.

## Method 1: Deploying as a Native Rust Binary

This method involves cross compiling the application and then moving the binary onto the Raspberry Pi.

### Build prerequisites
- `brew install rustup zig llvm`
- `cargo install cargo-zigbuild`
- `rustup target add aarch64-unknown-linux-musl`

### Systemd setup
Copy the systemd service file**:
```bash
scp deploy/monolith.service rs@raspberrypi.local:/etc/systemd/system/
```

### Certbot setup
TODO!

### TLS file permissions
Under this method the server runs as a non-root user, so it can't traverse `/etc/letsencrypt/...` and read the key.
- This repo includes a helper at `deploy/monolith_copy.sh` to copy certs into `/etc/monolith/tls` with correct permissions and restart the service.
- You can use it either as a certbot deploy-hook or run it manually.

Run it once manually (as root) after obtaining certs:
```bash
sudo SERVICE_USER=rs SERVICE_NAME=monolith DEST_DIR=/etc/monolith/tls \
  LINEAGE=/etc/letsencrypt/live/blobfishapp.duckdns.org \
  bash deploy/monolith_copy.sh
```

Install as a certbot deploy-hook so renewals update the copies automatically:
```bash
sudo install -m 0755 deploy/monolith_copy.sh /etc/letsencrypt/renewal-hooks/deploy/monolith_copy.sh
```

### Build and copy to Raspberry Pi
Build the binary:
```bash
cargo zigbuild --release --target aarch64-unknown-linux-musl
```
Copy the binary onto the Raspberry Pi:
```bash
scp target/release/prod rs@raspberrypi.local:/usr/local/bin/monolith-backend
```

### Restart service on Raspberry Pi
Ssh onto the Pi:
```bash
ssh rs@raspberrypi.local
```

Reload systemd to pick up the new unit:
```bash
sudo systemctl daemon-reload
```

Enable and start the service:
```bash
sudo systemctl enable --now monolith
```

## Database Setup

No manual database setup is required. On server startup, the application will:
- Create the SQLite database file if it does not exist (default: `./blobfishapp.sqlite`).
- Apply all pending migrations from the `migrations/` directory automatically.

This applies to both native and Docker deployments.

---

## Common Configuration: Let's Encrypt

Both deployment methods require TLS certificates to be present on the host machine. This implementation uses `axum_server::tls_rustls::RustlsConfig` to serve HTTPS.

The certificate and key files are expected to be at:
- `/etc/letsencrypt/live/blobfishapp.duckdns.org/fullchain.pem`
- `/etc/letsencrypt/live/blobfishapp.duckdns.org/privkey.pem`

You can obtain these using `certbot`.
