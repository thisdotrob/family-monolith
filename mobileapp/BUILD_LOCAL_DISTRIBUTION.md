# Local Build and Self-Hosted Distribution (Android & iOS)

This guide shows how to build the mobile apps locally and host the artifacts on your Rust server so your family can install via links — without app stores and without waiting in the EAS cloud queue.

The server already exposes routes for downloads. Place files on disk under:

- `server/static/downloads/android/<appId>/<filename>.apk`
- `server/static/downloads/ios/<appId>/<filename>.ipa`
- `server/static/downloads/ios/<appId>/manifest.plist`

Currently, the mobile scaffold ships a single app: `placeholder`.

## Android (APK) — Local Build and Host

New architecture is disabled for local builds (app.json: `newArchEnabled: false`). Lint is disabled for release via a config plugin (plugins/disable-android-lint.js).

1. Build APK locally (uses the `placeholder` profile configured for APK):

```bash
npm run build:android:local:placeholder
```

2. Copy the APK to the server’s download folder (adjust host/path as needed):

```bash
# Example: copy to Rust server static downloads directory
scp ./app-placeholder.apk rs@raspberrypi.local:/home/rs/monolith/static/downloads/android/placeholder/app.apk
```

3. Share the install link (Android device must allow "Install unknown apps"):

- `https://blobfishapp.duckdns.org/downloads/android/placeholder/app.apk`

## iOS (IPA) — Local Build, Manifest, and Host

Quick script (from mobileapp/):

iOS Ad Hoc distribution requires:

- Apple Developer Program
- Ad Hoc provisioning profile that includes each device’s UDID (family devices)

1. Build IPA locally (macOS/Xcode required):

```bash
npm run build:ios:local:placeholder
```

2. Create a manifest.plist (example below). Replace:

- IPA URL
- bundle-identifier (e.g., com.example.placeholder)
- bundle-version (e.g., 1.0.0)
- title (e.g., Placeholder)

Save as `manifest.plist`.

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>items</key>
  <array>
    <dict>
      <key>assets</key>
      <array>
        <dict>
          <key>kind</key>
          <string>software-package</string>
          <key>url</key>
          <string>https://blobfishapp.duckdns.org/downloads/ios/placeholder/app.ipa</string>
        </dict>
      </array>
      <key>metadata</key>
      <dict>
        <key>bundle-identifier</key>
        <string>com.example.placeholder</string>
        <key>bundle-version</key>
        <string>1.0.0</string>
        <key>kind</key>
        <string>software</string>
        <key>Placeholder</key>
        <string>Placeholder</string>
      </dict>
    </dict>
  </array>
</dict>
</plist>
```

3. Copy the IPA and manifest to the server:

```bash
scp ./app-placeholder.ipa rs@raspberrypi.local:/home/rs/monolith/static/downloads/ios/placeholder/app.ipa
scp ./manifest-placeholder.plist rs@raspberrypi.local:/home/rs/monolith/static/downloads/ios/placeholder/manifest.plist
```

4. Share the iOS install link (must open in Safari on device):

- `itms-services://?action=download-manifest&url=https://blobfishapp.duckdns.org/downloads/ios/placeholder/manifest.plist`

5. First-time install on iOS requires trusting the developer profile:

- Settings → General → VPN & Device Management → Developer App → Trust

## Notes & Troubleshooting

- Using npx vs global eas-cli:
  - The scripts use `npx -y eas-cli` so you don’t need a global install.
  - Alternatively, install globally: `npm i -g eas-cli` and run `eas build ...` directly.

- HTTPS is required for iOS itms-services links (use a proper certificate on your domain).
- iOS installs will only work on devices included in the Ad Hoc provisioning profile (UDIDs).
- Android devices must allow installing apps from the browser/File manager.
- Content types:
  - APK → `application/vnd.android.package-archive`
  - IPA → `application/octet-stream`
  - PLIST → `application/xml`
