# Serving Multiple Web Apps

The server can serve multiple built web apps under different routes.

Build each app with Vite by setting `VITE_APP_ID=<appId>`:

```bash
VITE_APP_ID=placeholder npm --prefix webapp run build
VITE_APP_ID=myapp npm --prefix webapp run build
```

Copy the built folders into `server/static/<appId>/` (or configure CI to do this).

- Example: `webapp/dist/placeholder` -> `server/static/placeholder`
- Example: `webapp/dist/myapp` -> `server/static/myapp`

Routes:
- `/:appId` -> serves `server/static/<appId>/index.html` if it exists; otherwise 404
- `/:appId/*` -> attempts to serve a file under `server/static/<appId>/<path>`; if not found, attempts `server/static/<appId>/assets/<path>`; otherwise 404

Notes:
- There is no root fallback at `/`. Requests to `/` will return 404.
- For client-side routing (deep links), you may need to configure your app to handle hash-based routing or ensure links are generated from the root app path.

This enables URLs like `/placeholder`, `/groceries`, `/trips`, etc.
