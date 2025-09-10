# Self-Hosted Mobile App Downloads

You can host mobile app artifacts on the server under `server/static/downloads/` and serve them via routes:

- Android APK: `/downloads/android/<appId>/<filename>.apk`
- iOS:
  - IPA: `/downloads/ios/<appId>/<filename>.ipa`
  - Manifest (for itms-services): `/downloads/ios/<appId>/manifest.plist`

Place files on the host filesystem at:

- `server/static/downloads/android/<appId>/<filename>.apk`
- `server/static/downloads/ios/<appId>/<filename>.ipa`
- `server/static/downloads/ios/<appId>/manifest.plist`

Example iOS manifest.plist (rename values):

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
          <string>https://your.domain/downloads/ios/placeholder/app.ipa</string>
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

Install links:

- Android: `https://your.domain/downloads/android/placeholder/app.apk`
- iOS (Safari): `itms-services://?action=download-manifest&url=https://your.domain/downloads/ios/placeholder/manifest.plist`

Notes:
- iOS requires Ad Hoc provisioning and registered device UDIDs.
- Serve over HTTPS. iOS will not install via plain HTTP URLs.
