# Web Multi-App Usage

This scaffold builds per-app bundles and serves them from the Rust server under `/:appId`.

## Build per app

```bash
# Build the app bundle with a specific app id
VITE_APP_ID=placeholder npm run build
# Output: dist/placeholder
```

- The build uses `base: /<appId>/` so asset URLs are rooted under the app path.
- The HTML `<title>` is injected to a Title Cased version of the app id at build time and also set at runtime.

## Select app in dev

```bash
VITE_APP_ID=placeholder npm run dev
```

The app module is chosen by `VITE_APP_ID` and resolved using `import.meta.glob('@apps-web/*/index.ts', { eager: true })`.

## Deploy to server

- Copy `webapp/dist/<appId>` into `server/static/<appId>`.
- The server routes:
  - `/:appId` -> serves `static/<appId>/index.html`
  - `/:appId/*` -> serves `static/<appId>/<path>` or `static/<appId>/assets/<path>`

## Create a new app

1. Create a directory under `apps/web/<newAppId>` with an `index.ts` that exports a default component.

```ts
// apps/web/myapp/index.ts
export { default } from './HomePage';
```

2. Build and deploy it:

```bash
VITE_APP_ID=myapp npm run build
cp -R dist/myapp ../server/static/myapp
```

3. Access it at `/:appId` (e.g., `/myapp`).
