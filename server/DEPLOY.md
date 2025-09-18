# Deployment

This document outlines deployment for the backend.

## Method 1: Deploying as a Native Rust Binary

This method involves cross compiling the application and then moving the binary onto the Raspberry Pi.

### Build prerequisites
- `brew install rustup zig llvm`
- `cargo install cargo-zigbuild`
- `rustup target add aarch64-unknown-linux-musl`

### Systemd setup
Copy the systemd service file:
```bash
scp deploy/monolith.service rs@raspberrypi.local:/tmp/monolith.service
ssh rs@raspberrypi.local "sudo mv /tmp/monolith.service /etc/systemd/system/
```

Create the working directory:
```bash
ssh rs@raspberrypi.local "mkdir -p ~/monolith"
```

### HTTPS setup (on the Raspberry Pi)
Install Certbot system dependencies:
```bash
sudo apt install python3 python3-dev python3-venv libaugeas-dev gcc
```

Set up virtual env:
```bash
sudo python3 -m venv /opt/certbot/
sudo /opt/certbot/bin/pip install --upgrade pip
```

Install certbot:
```bash
sudo /opt/certbot/bin/pip install certbot
sudo ln -s /opt/certbot/bin/certbot /usr/bin/certbot
```

Set up forwarding for port 80 on the router.

Request the certificates:
```bash
sudo certbot certonly --standalone -d blobfishapp.duckdns.org --keep-until-expiring --agree-tos --non-interactive --email this.rob@protonmail.com
```

The server runs as a non-root user, so it can't traverse `/etc/letsencrypt/...` and read the key.
This repo includes a helper at `deploy/copy_tls_certs.sh` to copy certs into `/etc/monolith/tls` with correct permissions and restart the service.

Copy the script to the raspberry pi:
```bash
scp deploy/copy_tls_certs.sh rs@raspberrypi.local:~/
```

Run it once manually (as root) after obtaining certs:
```bash
sudo SERVICE_USER=rs SERVICE_NAME=monolith DEST_DIR=/etc/monolith/tls \
  LINEAGE=/etc/letsencrypt/live/blobfishapp.duckdns.org \
  bash ~/copy_tls_certs.sh
```

Install as a certbot deploy-hook so renewals update the copies automatically:
```bash
sudo install -m 0755 ~/copy_tls_certs.sh /etc/letsencrypt/renewal-hooks/deploy/copy_tls_certs.sh
```

### Build and copy to Raspberry Pi
Build the binary:
```bash
cargo zigbuild --release --target aarch64-unknown-linux-musl
```
Copy the binary onto the Raspberry Pi:
```bash
scp target/aarch64-unknown-linux-musl/release/prod rs@raspberrypi.local:/tmp/monolith-backend
ssh rs@raspberrypi.local "sudo mv /tmp/monolith-backend /usr/local/bin/monolith-backend && sudo chmod +x /usr/local/bin/monolith-backend"
```

### Restart service on Raspberry Pi
Reload systemd to pick up the new unit:
```bash
ssh rs@raspberrypi.local "sudo systemctl daemon-reload"
```

Enable and start the service:
```bash
ssh rs@raspberrypi.local "sudo systemctl enable --now monolith"
```

Check service status and logs:
```bash
ssh rs@raspberrypi.local sudo systemctl status monolith.service
ssh rs@raspberrypi.local sudo journalctl -u monolith.service
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
scp deploy/monolith.service rs@raspberrypi.local:/tmp/monolith.service
ssh rs@raspberrypi.local "sudo mv /tmp/monolith.service /etc/systemd/system/
