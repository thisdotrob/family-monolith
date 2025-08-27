# Deployment

This document outlines two methods for deploying the monolith backend: as a native Rust binary or as a Docker container. Both methods use systemd to manage the service.

---

## Method 1: Deploying as a Native Rust Binary

This method involves compiling the application on the target machine (e.g., a Raspberry Pi) and running it directly.

### Prerequisites
- **Rust**: Install Rust using `rustup`:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
- **Build Tools**: You may need `build-essential` or equivalent packages.

### Setup
1.  **Clone the repository and build the binary**:
    ```bash
    git clone <repository-url>
    cd <repository-directory>
    cargo build --release
    ```
2.  **Copy the binary** to a location in your `PATH`, for example:
    ```bash
    sudo cp target/release/prod /usr/local/bin/monolith-backend
    ```
3.  **Copy the systemd service file**:
    ```bash
    sudo cp deploy/monolith.service /etc/systemd/system/
    ```
4.  **Reload systemd to pick up the new unit**:
    ```bash
    sudo systemctl daemon-reload
    ```
5.  **Enable and start the service**:
    ```bash
    sudo systemctl enable --now monolith
    ```

---

## Method 2: Deploying as a Docker Container

This method uses the Docker image built by the CI pipeline. It is the recommended method as it doesn't require compiling on the target machine.

### Prerequisites
- **Docker**: Install Docker on your system.
- **Let's Encrypt Certificates**: You still need valid certificates on the host machine.

### Setup
1.  **Copy the Docker systemd service file**:
    ```bash
    sudo cp deploy/monolith-docker.service /etc/systemd/system/
    ```
2.  **Reload the systemd daemon** to recognize the new service:
    ```bash
    sudo systemctl daemon-reload
    ```
3.  **Enable and start the service**:
    ```bash
    sudo systemctl enable --now monolith-docker
    ```
The service will automatically pull the latest image from `ghcr.io` and run it.

---

## Common Configuration: Let's Encrypt

Both deployment methods require TLS certificates to be present on the host machine. This implementation uses `axum_server::tls_rustls::RustlsConfig` to serve HTTPS.

The certificate and key files are expected to be at:
- `/etc/letsencrypt/live/blobfishapp.duckdns.org/fullchain.pem`
- `/etc/letsencrypt/live/blobfishapp.duckdns.org/privkey.pem`

You can obtain these using `certbot`. For the Docker deployment, these files are mounted into the container.
