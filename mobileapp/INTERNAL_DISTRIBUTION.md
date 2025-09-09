# Mobile Internal Distribution (No App Store Required)

This guide explains how to distribute the React Native (Expo) mobile apps to family members without publishing to the app stores, using Expo Application Services (EAS) Internal Distribution.

It works per app id (APP_ID) using EAS build profiles defined in `mobileapp/eas.json` and app metadata defined in `mobileapp/app.config.ts`.

## Prerequisites

- Install the EAS CLI (once):
  - `npm i -g eas-cli`
  - Verify: `eas --version`
- Login to Expo: `eas login`
- In this repo, the mobile scaffold selects the app module by `APP_ID` via `mobileapp/src/selectMobileApp.ts` and per-app metadata via `mobileapp/app.config.ts`.
- EAS build profiles are defined in `mobileapp/eas.json` (placeholder).

## Overview

- Android: Distribute an APK (or AAB) via a private Expo install link. Family members enable "Install unknown apps" and install.
- iOS: Use Ad Hoc distribution (Apple Developer Program required). Family members must register device UDIDs. Expo generates a private install page for each build; they install from Safari and trust the developer.

## Build Profiles (per app)

`mobileapp/eas.json` includes examples:

```json
{
  "build": {
    "placeholder": { "env": { "APP_ID": "placeholder" }, "distribution": "internal" },
    "groceries": { "env": { "APP_ID": "groceries" }, "distribution": "internal" },
    "trips": { "env": { "APP_ID": "trips" }, "distribution": "internal" }
  }
}
```

These map to the app metadata in `mobileapp/app.config.ts` (name/slug/bundle/package) and the app module registry in `mobileapp/src/selectMobileApp.ts`.

## Build an Internal App

Run from the repo root (or `cd mobileapp`). Choose a profile per app.

- Android:
  ```bash
  eas build --profile placeholder --platform android
  ```
- iOS (Ad Hoc):
  ```bash
  eas build --profile placeholder --platform ios
  ```

EAS will print a build page URL. When builds complete, you get install links you can share with family.

## iOS Device Registration (Ad Hoc)

Apple requires registering device UDIDs for Ad Hoc builds (limit ~100 devices/year per Apple Dev account).

- Send each family member the Expo device registration link (available on the install page or at https://expo.dev/register-device).
- They follow the instructions to capture the UDID.
- Re-run the iOS build so EAS includes new UDIDs in the provisioning profile, or let EAS handle it during build when prompted.

Install on iOS:

- Open the build install link in Safari on the device.
- After install, trust the developer: Settings → General → VPN & Device Management → Developer App → Trust.

## Android Install (APK)

- Open the build install link on the device.
- Download the APK and install it. If prompted, enable "Install unknown apps" for the browser/Files app.

## Updating Apps

Two options:

1. Rebuild and share a new internal build link:
   - `eas build --profile placeholder --platform ios`
   - `eas build --profile placeholder --platform android`

2. Over-the-Air (OTA) Updates with EAS Update (configured)
   - OTA is enabled via `expo-updates`. The placeholder app uses channel `family-placeholder` (see `app.config.ts`).
   - Publish JS updates without rebuilding binaries (pick the matching branch/channel):
     ```bash
     cd mobileapp
     # Example for placeholder app
     eas update --branch family-placeholder --message "UI fix"
     ```
   - Devices check for updates on app load (`checkAutomatically: ON_LOAD`).
   - Note: OTA updates can’t change native modules.

## Local Development Per App

- Run with a specific app id locally:
  ```bash
  cd mobileapp
  APP_ID=placeholder npx expo start
  ```
- The resolver `src/selectMobileApp.ts` loads the appropriate module under `apps/mobile/<APP_ID>`.

## Adding Another Family App (future)

If you choose to add more apps later:

- Create `apps/mobile/<appId>` with an `index.ts` exporting the root component.
- Reintroduce a dynamic resolver keyed by `APP_ID` (or a codegen registry).
- Add per-app metadata in `app.config.ts` and an EAS profile in `eas.json`.
- Build and distribute internal builds per profile.

## Troubleshooting

- iOS install fails / profile invalid:
  - Ensure the device UDID is registered and re-run the iOS build.
  - Trust the developer profile under Settings.
- Android can’t install APK:
  - Enable "Install unknown apps" for the browser/File manager.
- Wrong app content shows:
  - Ensure the correct profile is used (sets `APP_ID`), and the module is in `selectMobileApp.ts`.
- Need separate icons/names per app:
  - Configure `app.config.ts` per-app metadata (icon, splash, name, slug, scheme).

## Security Notes

- Treat install links as private. Anyone with the link could attempt to install (Android) or request device registration (iOS).
- For iOS, device installs only work if the UDID is included in the provisioning profile.

---

That’s it! This setup lets your family install your apps privately, without publishing to the App Store or Play Store.
