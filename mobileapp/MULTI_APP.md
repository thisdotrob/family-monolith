# Mobile Multi-App Usage

The mobile scaffold reuses shared auth and Apollo, and loads a specific app module under `apps/mobile/<appId>`.

## Select app in dev

Use an env var for `APP_ID` (to be wired in a future step) or hardcode the import to `@apps-mobile/<appId>`.

Current POC uses `@apps-mobile/placeholder`.

## Create a new mobile app

1. Create `apps/mobile/<newAppId>` with an `index.ts` exporting the app's root component.
2. Update imports in `mobileapp/src/App.tsx` to point to the new app while we wire up dynamic selection.

## EAS profiles

Define EAS build profiles with unique `slug`, `ios.bundleIdentifier`, and `android.package` per app. In each profile, set an `APP_ID` env var and configure app-specific metadata in `app.json` or `app.config.ts`.
