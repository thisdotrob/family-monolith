# Multi-App Monorepo Plan (Web + Mobile sharing a Rust GraphQL backend)

This plan describes a scalable structure that enables many separate web and mobile applications to reuse a common authentication and data access scaffold, all backed by the Rust GraphQL server in `./server`.

Goals:
- Each web app should be deployable at its own URL (domain or subpath).
- Each mobile app should be installable separately (unique app id, bundle id, and branding).
- Reuse a shared scaffold for auth, Apollo GraphQL, and cross-cutting concerns.
- Only app-specific UI and logic should be implemented per-app.

## High-level Architecture

- `server/` (unchanged): Rust GraphQL server. Auth endpoints remain `/v1/graphql/auth`, app endpoints `/v1/graphql/app`.
- `shared/`: Cross-platform (web + mobile) TypeScript modules:
  - `contexts/AuthContext.tsx` (auth state, token storage hooks)
  - `apollo/createApolloClient.ts` (auth link, refresh link, error handling)
  - `graphql/` (queries and mutations)
- `webapp/`: Web scaffold (React + Vite + Tailwind):
  - Provides the common runtime wiring: AuthProvider, ApolloProvider, loading states, login page.
  - Delegates the "protected" section (post-login) to a selected app module.
- `mobileapp/`: Mobile scaffold (React Native + Expo):
  - Provides the common runtime wiring: AuthProvider, ApolloProvider, loading states, login page.
  - Delegates the "protected" section (post-login) to a selected app module.
- `apps/`: Houses app-specific UI for both platforms:
  - `apps/web/<appName>/` exports a default React component that renders the protected UI for that app.
  - `apps/mobile/<appName>/` exports a default React component that renders the protected UI for that app.

This cleanly separates reusable scaffolds from per-app UI.

## Repository Layout

- `server/`
- `shared/`
- `webapp/` (scaffold)
- `mobileapp/` (scaffold)
- `apps/`
  - `web/`
    - `placeholder/` (proof-of-concept web app)
      - `index.ts` (export default component)
      - `HomePage.tsx` (app UI)
  - `mobile/`
    - `placeholder/` (proof-of-concept mobile app)
      - `index.ts` (export default component)
      - `HomePage.tsx` (app UI)
- `docs/`
  - `multi-app-structure-plan.md` (this file)

## Web Scaffold Integration Strategy

- Use a TypeScript path alias (in `webapp/tsconfig.app.json`) to import app modules from `../apps/web/*`:
  - `"@apps-web/*": ["../apps/web/*"]`
- The web scaffold owns:
  - AuthProvider, ApolloProvider, token storage (LocalStorage), global loading, Login page, and global styles.
- The web scaffold imports the selected app’s default export and renders it when authenticated. For example:
  - `import HomePage from '@apps-web/placeholder'`
- Deployment options for separate web apps (choose one or mix):
  1. Multi-entry Vite build (one HTML entry per app) with separate `base` paths; deploy each `dist/<appName>` to a distinct URL/subpath.
  2. Separate builds per app using an env variable to select the app module and output directory (e.g., `APP_ID=finance npm run build`).
  3. Single SPA with dynamic `import.meta.glob` + client-side routing, published at a single domain with subpaths. Useful for internal portals; less ideal for totally separate public URLs.

Recommended: Start with (2) for simplicity. For each app, set `VITE_APP_ID` and build, producing separate static bundles per app for hosting.

## Mobile Scaffold Integration Strategy

- Use a TypeScript path alias (in `mobileapp/tsconfig.json`) to import app modules from `../apps/mobile/*`:
  - `"@apps-mobile/*": ["../apps/mobile/*"]`
- The mobile scaffold owns:
  - AuthProvider, ApolloProvider, token storage (AsyncStorage), global loading, Login page, Paper provider, and SafeArea handling.
- The mobile scaffold imports the selected app’s default export and renders it when authenticated. For example:
  - `import HomePage from '@apps-mobile/placeholder'`
- To install mobile apps separately, create separate EAS profiles and app identifiers:
  - Use `app.json` or `app.config.ts` with environment-based overrides for `slug`, `bundleIdentifier` (iOS), and `package` (Android).
  - Define EAS profiles (e.g., `placeholder`, `groceries`, `trips`) that set env vars like `APP_ID` and unique bundle ids.
  - At build time, `APP_ID` toggles which app module is imported.

## Configuration: Selecting an App at Build Time

The web scaffold selects the app via `VITE_APP_ID` at dev/build time. To generalize the mobile scaffold:

- Web (Vite):
  - Use `import.meta.env.VITE_APP_ID` and an index file to map IDs to modules:
    - Use `const modules = import.meta.glob('../../apps/web/**/index.ts', { eager: true })`
    - Resolve `VITE_APP_ID` to a module and render it.
  - Configure `vite.config.ts` to set `base` and output directories per `VITE_APP_ID`.

- Mobile (Expo):
  - Use `process.env.APP_ID` exposed via `app.config.ts` or `expo-constants`.
  - For type-safety, create a small resolver that maps `APP_ID` to a module using a static object or `require` map.
  - Define multiple EAS profiles with `env.APP_ID` and unique `ios.bundleIdentifier` / `android.package`.

## Auth, Token Refresh, and GraphQL

- Continue to use the shared modules:
  - `AuthProvider` for token lifecycle, logout, and state.
  - `createApolloClient` for auth headers and automatic refresh token rotation on `TOKEN_EXPIRED`.
- Per-app components can freely use `useQuery`, `useMutation`, and fragments with `@shared/graphql/*`.

## Routing

- Web: The scaffold can optionally provide a simple router wrapper for per-app routes. Each app can export its own router subtree (e.g., `react-router` or a simple local state) without affecting the scaffold.
- Mobile: Each app can embed its own navigation stack (React Navigation) within its root component.

## Testing

- Keep unit and component tests inside each scaffold.
- Add per-app tests in `apps/<platform>/<appName>/__tests__`.
- Mock GraphQL using Apollo MockedProvider in React tests or MSW.

## Deployment

- Web: Build per app (`VITE_APP_ID=<appId> npm run build`) and deploy `dist/<appId>/` to the server under `server/static/<appId>` (served at `/:appId`).
- Mobile: Use EAS to build and submit each app variant with unique identifiers.
- Backend: No changes; ensure proper CORS for all web origins.

## Proof-of-Concept (This PR)

- Create `apps/web/placeholder` and `apps/mobile/placeholder`.
- Move current `HomePage` from `webapp/src/pages/HomePage.tsx` to `apps/web/placeholder/HomePage.tsx` and export via `apps/web/placeholder/index.ts`.
- Move current `HomePage` from `mobileapp/src/pages/HomePage.tsx` to `apps/mobile/placeholder/HomePage.tsx` and export via `apps/mobile/placeholder/index.ts`.
- Update TypeScript path aliases in `webapp` and `mobileapp` to point to the new app folders.
- Update `webapp/src/App.tsx` and `mobileapp/src/App.tsx` to import the app from the new location.
- Keep login screens and shared providers in the scaffolds.

## Next Steps

- Add dynamic app selection via env vars (`VITE_APP_ID` for web, `APP_ID` for mobile) with a tiny resolver.
- Introduce a `webapp/apps.config.ts` and `mobileapp/apps.config.ts` to map `APP_ID` to modules and optional metadata (name, icon, theme).
- For web, set up per-app build tasks to produce separate artifacts (already supported via `VITE_APP_ID`).
- For mobile, add EAS profiles for multiple app targets with unique bundle identifiers and app names.
- Document developer workflows (create new app, run locally, build/deploy).
