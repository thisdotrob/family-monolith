# Mobile App (Expo) - Multi App + Internal Distribution + OTA Updates

## Build Profiles

See `eas.json` for profiles per app: placeholder, groceries, trips.

- Build (internal distribution):
  - iOS: `eas build --profile placeholder --platform ios`
  - Android: `eas build --profile placeholder --platform android`
- Submit to stores (optional):
  - iOS: `eas submit --profile placeholder --platform ios --latest`
  - Android: `eas submit --profile placeholder --platform android --latest`

## Select App Module

- Resolver at `src/selectMobileApp.ts` picks the app by `APP_ID` (from `app.config.ts` -> `extra.APP_ID`).
- Add imports and registry entries for new apps under `apps/mobile/<appId>`.

## Per-App Metadata (and OTA Channel)

- `app.config.ts` sets name, slug, bundle identifiers, and OTA updates channel per app.
- OTA updates (expo-updates) are enabled and set to check automatically on load.

## Internal Distribution

- See `INTERNAL_DISTRIBUTION.md` for step-by-step installation (Android APK, iOS Ad Hoc with UDIDs).

## OTA Updates

- Publish a JS/UI update to the appropriate channel:
  ```bash
  eas update --branch family-placeholder --message "Fix header"
  ```
- Note: OTA updates can’t change native modules.
