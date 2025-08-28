#!/usr/bin/env bash
# Monolith cert deploy hook and one-off copier
# - When run by certbot as a deploy hook, it uses $RENEWED_LINEAGE
# - When run manually, it falls back to a default lineage path
# You can override via env vars:
#   SERVICE_NAME   (default: monolith)
#   SERVICE_USER   (default: rs)
#   DEST_DIR       (default: /etc/monolith/tls)
#   LINEAGE        (default: /etc/letsencrypt/live/blobfishapp.duckdns.org)

set -euo pipefail

SERVICE_NAME="${SERVICE_NAME:-monolith}"
SERVICE_USER="${SERVICE_USER:-rs}"
DEST_DIR="${DEST_DIR:-/etc/monolith/tls}"
LINEAGE="${RENEWED_LINEAGE:-${LINEAGE:-/etc/letsencrypt/live/blobfishapp.duckdns.org}}"

# Ensure destination dir exists with secure perms
install -d -m 0750 -o root -g "$SERVICE_USER" "$DEST_DIR"

# Copy renewed/current certs with secure perms
install -m 0640 -o root -g "$SERVICE_USER" \
  "$LINEAGE/fullchain.pem" "$DEST_DIR/fullchain.pem"
install -m 0640 -o root -g "$SERVICE_USER" \
  "$LINEAGE/privkey.pem" "$DEST_DIR/privkey.pem"

# Restart service to pick up new certs (best-effort)
systemctl restart "$SERVICE_NAME" || true

echo "Monolith TLS certs copied to $DEST_DIR and service '$SERVICE_NAME' restarted (if available)."