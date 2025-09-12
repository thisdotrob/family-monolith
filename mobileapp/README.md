# Mobile App (Expo) — Placeholder App + Deployment to family devices

This mobile scaffold currently ships a single app: `placeholder`.

- App selection: hardcoded to `placeholder` (see `src/selectMobileApp.ts`)
- App metadata: set in `app.config.ts` (name, slug, bundle identifiers, OTA channel)
- Android deployment: local building with `eas-cli` and self hosted install links
- iOS deployment: local building and sideloading in Xcode

## Select App Module

- The resolver at `src/selectMobileApp.ts` returns the placeholder app.
- `app.config.ts` is hardcoded to the placeholder app and injects `extra.APP_ID = 'placeholder'`.

## Deployment to family devices

See `BUILD_LOCAL_DISTRIBUTION.md`.

## Local Development

```bash
cd mobileapp
npx expo start
```
