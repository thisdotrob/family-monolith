# monolith-backend

[![CI](https://github.com/thisdotrob/family-monolith-backend/actions/workflows/ci.yml/badge.svg)](https://github.com/thisdotrob/family-monolith-backend/actions/workflows/ci.yml)

## Serving Multiple Web Apps

This server can serve multiple static web apps built from the `webapp` scaffold.

- Build per app: `VITE_APP_ID=<appId> npm --prefix webapp run build`
- Copy to server: `cp -R webapp/dist/<appId> server/static/<appId>`
- Access at: `/:appId` (e.g., `/placeholder`)
- Details: see `server/STATIC_APPS.md`

## Docker

### Pull
```bash
docker pull ghcr.io/thisdotrob/family-monolith-backend:latest
```

### Run
```bash
docker run -p 8080:8080 ghcr.io/thisdotrob/family-monolith-backend:latest
```
