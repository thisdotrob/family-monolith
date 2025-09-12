# Local Build and Self-Hosted Distribution (Android & iOS)

This guide shows how to build the mobile apps locally and deploy to family devices either by sideloading with xcode (ios) or hosting the artifact on the Rust server with install links (android).

Currently, the mobile scaffold ships a single app: `placeholder`.

## Android (APK) — Local Build and Hosted Install Link

### Configuration notes
New architecture is disabled for local builds (app.json: `newArchEnabled: false`). Lint is disabled for release via a config plugin (plugins/disable-android-lint.js).

### Build & Distributrion steps

1. Build APK locally (uses the `placeholder` profile configured for APK):

```bash
npm run build:android:placeholder
```

2. Copy the APK to the server’s downloads folder:

```bash
scp ./app-placeholder.apk rs@raspberrypi.local:/home/rs/monolith/static/downloads/android/placeholder/app.apk
```

3. Share the install link (Android device must allow "Install unknown apps"):

- `https://blobfishapp.duckdns.org/downloads/android/placeholder/app.apk`

## iOS — Local Build and Sideloading with Xcode

For iOS, we prebuild with expo then build and sideload in Xcode. This avoids paying for App Store distribution, TestFlight, or over-the-air installation methods.

### iPhone requiements

- Must have developer mode enabled.

### Build & Distributrion steps

1. Prebuild the app with expo:

```bash
npm run prebuild:ios:placeholder
```

2. Open prebuilt project in Xcode:

```bash
npm run buildui:ios:placeholder
```

3. Build the project in Xcode:

**Product -> Build** in top menu.

4. Create archive:

**Product -> Archive** in top menu.

5. Export the `.app` file:

- In the Organizer, right-click your archive                                                                        │
- Select "Show in Finder"                                                                                           │
- Right-click the `.xcarchive` file                                                                                   │
- Select "Show Package Contents"                                                                                    │
- Navigate to `Products/Applications/`
- Drag the `.app` file to the device in Xcode's Devices window

6. Trust the Developer Certificate:

After installation, you need to trust the developer certificate on the iOS device:
- Go to **Settings → General → VPN & Device Management**
- Under **Developer App**, tap your Apple ID
- Tap **Trust "[Your Apple ID]"**
- Tap **Trust** in the confirmation dialog

The app should now be available on your home screen and ready to use.
