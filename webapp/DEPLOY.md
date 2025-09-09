# Web Apps Deployment (hosted at "/:appId")

This document explains how to build per-app bundles and deploy them so the Rust server serves each app at a distinct path `/:appId`.

The server is configured to serve static files from a `static/` directory under its WorkingDirectory.

- WorkingDirectory (from `server/deploy/monolith.service`): `/home/rs/monolith`
- Static directory used by the server (see `server/src/server/mod.rs`): `/home/rs/monolith/static/<appId>`

If your service unit uses a different WorkingDirectory, adjust the paths accordingly.

## Script based deployment

The full manual instructions are documented in the following sections, but all you should need to do is:

```bash
# From webapp/ directory
VITE_APP_ID=<appId> npm run build
# then copy dist/<appId> to the server's static folder
npm run predeploy:pi    # ensure remote /home/rs/monolith/static exists
rsync -avz --delete dist/<appId>/ rs@raspberrypi.local:/home/rs/monolith/static/<appId>/
```

---

## 1) Build the web app per appId

On your development machine or CI runner:

```bash
cd webapp
npm ci  # or: npm install
VITE_APP_ID=<appId> npm run build
```

The production build will be created at `webapp/dist/`.

---

## 2) Deploy the build to the server's static directory

You have two common options: copy locally on the server, or upload from your dev machine using rsync/scp.

Before copying, ensure the destination directory exists on the server and is writable by the service user (here `rs`).

```bash
# On the server (one-time):
sudo -u rs mkdir -p /home/rs/monolith/static/<appId>
```

### Option A: Copy locally on the server

If you built on the server itself:

```bash
# From repository root after running the build above
tree webapp/dist  # optional: inspect output
rsync -av --delete webapp/dist/<appId>/ /home/rs/monolith/static/<appId>/

# Set ownership to the service user if needed (example: rs:rs)
sudo chown -R rs:rs /home/rs/monolith/static
```

### Option B: Upload from your dev machine using rsync (raspberry pi)

For your setup, the Raspberry Pi is reachable at `rs@raspberrypi.local`. The command below mirrors the content of `dist/` into the remote `static/` directory and deletes files that no longer exist locally.

```bash
# Run on your development machine, from repository root after building
rsync -avz --delete webapp/dist/<appId>/ rs@raspberrypi.local:/home/rs/monolith/static/<appId>/
```

Alternatively, using scp (no deletion of removed files):

```bash
scp -r webapp/dist/<appId>/* rs@raspberrypi.local:/home/rs/monolith/static/<appId>/
```

Shortcuts via npm scripts:

```bash
# From webapp/ directory
VITE_APP_ID=<appId> npm run build          # build locally
npm run predeploy:pi   # ensure remote /home/rs/monolith/static/<appId> exists
npm run deploy:pi      # rsync dist/<appId> to the Pi
# or in one go
npm run build:deploy:pi
```

---

## 3) Verify and serve

- The Rust server (Axum) serves each app at `/:appId`.
- Typically, no server restart is needed; files are served directly from the filesystem. If you have a CDN or proxy in front, you may need to clear its cache.

Quick checks:

```bash
# On the server
ls -l /home/rs/monolith/static/<appId>

# From your browser
https://blobfishapp.duckdns.org/<appId>
```

If the service is not yet running, follow the backend deployment steps in `server/DEPLOY.md`.

---

## Notes

- If you change the service WorkingDirectory, update the destination path accordingly (destination should always be `<WorkingDirectory>/static`).
- This project uses Vite. If your app relies on environment variables at build time, ensure they are provided (e.g., `VITE_*`) before running `npm run build`.
- SPA routing: the server currently serves static files as-is. If you add client-side routes (e.g., React Router) and need history API fallback to `index.html` for non-root paths, you may need to add a fallback handler to the server or ensure external routing rewrites in your reverse proxy.
