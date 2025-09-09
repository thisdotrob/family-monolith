# Mobile App (Expo) — Placeholder App + Internal Distribution + OTA Updates

This mobile scaffold currently ships a single app: `placeholder`.

- App selection: hardcoded to `placeholder` (see `src/selectMobileApp.ts`)
- App metadata: set in `app.config.ts` (name, slug, bundle identifiers, OTA channel)
- Internal distribution: via EAS Internal Distribution
- OTA updates: enabled via `expo-updates` on channel `family-placeholder`

## Build Profiles

See `eas.json` for the available profiles.

- Build (internal distribution):
  - iOS: `eas build --profile placeholder --platform ios`
  - Android: `eas build --profile placeholder --platform android`
- Submit to stores (optional):
  - iOS: `eas submit --profile placeholder --platform ios --latest`
  - Android: `eas submit --profile placeholder --platform android --latest`

## Select App Module

- The resolver at `src/selectMobileApp.ts` returns the placeholder app.
- `app.config.ts` is hardcoded to the placeholder app and injects `extra.APP_ID = 'placeholder'`.

## OTA Updates (expo-updates)

- Devices check for updates on app load.
- Publish a JS/UI update to the placeholder channel:
  ```bash
  cd mobileapp
  eas update --branch family-placeholder --message "UI fix"
  ```
- Note: OTA updates can’t change native modules.

## Internal Distribution

- See `INTERNAL_DISTRIBUTION.md` for step-by-step installation (Android APK, iOS Ad Hoc with UDIDs).

## Local Development

```bash
cd mobileapp
npx expo start
```

## Future Work (optional)

If you later choose to add additional apps under `apps/mobile/<appId>`, we can:

- Reintroduce a dynamic resolver keyed by `APP_ID`
- Add per-app EAS build profiles and per-app metadata in `app.config.ts`
- Provide a small codegen step to auto-register app modules
