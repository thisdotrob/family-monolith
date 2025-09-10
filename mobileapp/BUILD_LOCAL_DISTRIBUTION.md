# Local Build and Self-Hosted Distribution (Android & iOS)

This guide shows how to build the mobile apps locally and host the artifacts on your Rust server so your family can install via links — without app stores and without waiting in the EAS cloud queue.

The server already exposes routes for downloads:

- Android APK: `/downloads/android/<appId>/<filename>.apk`
- iOS artifacts: `/downloads/ios/<appId>/<filename>` (supports `.ipa` and `.plist`)

Place files on disk under:

- `server/static/downloads/android/<appId>/<filename>.apk`
- `server/static/downloads/ios/<appId>/<filename>.ipa`
- `server/static/downloads/ios/<appId>/manifest.plist`

Currently, the mobile scaffold ships a single app: `placeholder`.

## Prerequisites

- EAS CLI installed and logged in:
  - `npm i -g eas-cli`
  - `eas login`
- Local build environment option A (recommended): Docker-based EAS local builds
  - Install Docker and enable it
  - EAS will use a container to build without installing Android SDK/Xcode locally
- Local build environment option B: Native toolchains
  - Android: Java (JDK), Android SDK, NDK, Gradle installed/configured
  - iOS: macOS + Xcode + Apple Developer account for Ad Hoc signing (UDIDs required)

## Android (APK) — Local Build and Host

1. Build APK locally (uses the `placeholder` profile configured for APK):

```bash
cd mobileapp
# Build locally with Docker (recommended)
eas build --profile placeholder --platform android --local --non-interactive --output ./app-placeholder.apk

# Or build locally using your host toolchain
# eas build --profile placeholder --platform android --local --output ./app-placeholder.apk
```

2. Copy the APK to the server’s download folder (adjust host/path as needed):

```bash
# Example: copy to Rust server static downloads directory
scp ./app-placeholder.apk rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/android/placeholder/app.apk
```

3. Share the install link (Android device must allow "Install unknown apps"):

- `https://<your-domain-or-ip>/downloads/android/placeholder/app.apk`

## iOS (IPA) — Local Build, Manifest, and Host

iOS Ad Hoc distribution requires:

- Apple Developer Program
- Ad Hoc provisioning profile that includes each device’s UDID (family devices)

1. Build IPA locally (macOS/Xcode required):

```bash
cd mobileapp
# Use Docker builder if set up for iOS (macOS host still required)
eas build --profile placeholder --platform ios --local --non-interactive --output ./app-placeholder.ipa

# Alternative: Xcode Archive + Ad Hoc export to produce app-placeholder.ipa
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
          <string>https://<your-domain-or-ip>/downloads/ios/placeholder/app.ipa</string>
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
        <key>title</key>
        <string>Placeholder</string>
      </dict>
    </dict>
  </array>
</dict>
</plist>
```

3. Copy the IPA and manifest to the server:

```bash
scp ./app-placeholder.ipa rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/ios/placeholder/app.ipa
scp ./manifest.plist rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/ios/placeholder/manifest.plist
```

4. Share the iOS install link (must open in Safari on device):

- `itms-services://?action=download-manifest&url=https://<your-domain-or-ip>/downloads/ios/placeholder/manifest.plist`

5. First-time install on iOS requires trusting the developer profile:

- Settings → General → VPN & Device Management → Developer App → Trust

## Notes & Troubleshooting

- HTTPS is required for iOS itms-services links (use a proper certificate on your domain).
- iOS installs will only work on devices included in the Ad Hoc provisioning profile (UDIDs).
- Android devices must allow installing apps from the browser/File manager.
- If the server returns 404:
  - Verify file paths match:
    - Android: `server/static/downloads/android/placeholder/app.apk`
    - iOS: `server/static/downloads/ios/placeholder/app.ipa` and `manifest.plist`
  - Confirm the Rust server is running with these routes (already integrated):
    - `/downloads/android/:app_id/:filename`
    - `/downloads/ios/:app_id/:filename`
- Content types:
  - APK → `application/vnd.android.package-archive`
  - IPA → `application/octet-stream`
  - PLIST → `application/xml`

## Optional Automation Tips

- Add a small script to copy artifacts after local builds to the correct server path.
- Use rsync for incremental uploads:

```bash
rsync -avz ./app-placeholder.apk rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/android/placeholder/app.apk
rsync -avz ./app-placeholder.ipa rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/ios/placeholder/app.ipa
rsync -avz ./manifest.plist rs@raspberrypi.local:/home/rs/monolith/server/static/downloads/ios/placeholder/manifest.plist
```

That’s it — local builds, self-hosted downloads, and no waiting in the cloud queue.
