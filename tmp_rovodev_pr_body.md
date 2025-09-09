Summary
This PR introduces a multi-app structure so we can host multiple web and mobile apps sharing common auth and GraphQL scaffolding. It also updates the server to serve any built webapp at /:app_id and improves build-time/runtime configuration for app selection and titles.

What changed
- New app modules
  - apps/web/placeholder with the existing HomePage migrated out of webapp scaffold
  - apps/mobile/placeholder with the existing HomePage migrated out of mobile scaffold
- Web scaffold
  - Dynamic app selection via VITE_APP_ID using import.meta.glob to load @apps-web/*/index.ts
  - Vite base set to /<appId>/ and build outDir set to dist/<appId>
  - Title injected at build (HTML) and also set at runtime based on app id (Title Case)
  - Tailwind scans ../apps/web and ../shared (no safelist needed)
- Server (Axum)
  - Host multiple apps at /:app_id
  - GET /:app_id: serve static/<app_id>/index.html or 404
  - GET /:app_id/*: serve files under static/<app_id>/<path> or static/<app_id>/assets/<path>; else 404
  - Proper content-type detection for common file types
  - No root fallback at /
- Mobile scaffold
  - Path aliases and Metro config support for apps/mobile and shared imports
- Documentation
  - docs/multi-app-structure-plan.md: architecture, selection, builds, deploy guidance
  - server/STATIC_APPS.md: routes, no root fallback, asset path handling
  - server/README.md/DEPLOY.md: updated to mention multi-app static hosting
  - webapp/DEPLOY.md: per-app build and per-app deploy paths (VITE_APP_ID=<appId>)
  - webapp/MULTI_APP.md, mobileapp/MULTI_APP.md: quick start guides

How to test locally
- Web
  - Build placeholder app:
    VITE_APP_ID=placeholder npm --prefix webapp run build
  - Copy to server static:
    cp -R webapp/dist/placeholder server/static/placeholder
  - Start server:
    cargo run --bin dev
  - Visit:
    - http://localhost:4173/placeholder (app renders)
    - http://localhost:4173/placeholder/assets/... (assets load)
    - http://localhost:4173/placeholder/vite.svg (top-level file served)
- Mobile
  - Run:
    cd mobileapp && npm install && npx expo start
  - Confirm shared auth flow still works and HomePage renders from apps/mobile/placeholder

Deploy notes
- Build:
  VITE_APP_ID=<appId> npm --prefix webapp run build
- Deploy:
  rsync dist/<appId>/ to server/static/<appId> (paths documented in webapp/DEPLOY.md)
- Server will serve the app at /<appId> without additional changes.

Backwards compatibility
- No root fallback; requests to / now 404 by design
- Existing auth/GraphQL endpoints unchanged
- Tailwind and build config updated to support external app code

Checklist
- [x] Web: Build output per appId (base + outDir)
- [x] Web: Dynamic selection and title injection
- [x] Server: /:app_id routing with SPA handling and assets
- [x] Mobile: Path aliases + Metro config
- [x] Docs updated with new build and deploy flow

Optional labels
- enhancement, frontend, backend, mobile, docs
