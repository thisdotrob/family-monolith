You are an expert technical writer. Your task is to create the deployment documentation for both the web and mobile applications as specified.

**Commit Title:** `docs: create deployment documentation`

Create a new file named `DEPLOYMENT.md` in the root of the `monolith-frontend` directory.

**`DEPLOYMENT.md`:**
```markdown
# Deployment Instructions

This document provides instructions for deploying the web and mobile applications.

## Web Application (`webapp`)

The web application is a standard Vite React project. It needs to be bundled into a set of static assets (HTML, CSS, JavaScript) which can then be served by the backend.

### 1. Build the Application

Navigate to the web application's directory:
```bash
cd webapp
```

Run the build command. This will compile the TypeScript and React code and bundle everything into a `dist` directory.
```bash
npm run build
```

### 2. Copy Assets to Backend

After the build is complete, a `dist` directory will be created inside the `webapp` folder. This directory contains the `index.html` file and `assets/` subdirectory with all the necessary CSS and JavaScript files.

**You must manually copy the entire contents of this `dist` directory into the `static/` directory of the backend Rust server.**

The backend is configured to serve files from its `static/` directory. Once the files are copied, the web application will be live and accessible through the backend's address.

## Mobile Application (`mobileapp`)

The mobile application is built with Expo and is intended for distribution to family members via TestFlight on iOS.

### 1. Prerequisites

Before you can build and distribute the app, you need:
- An active **Apple Developer Program** membership.
- **Expo CLI** installed on your machine (`npm install -g expo-cli`).
- **EAS CLI** installed on your machine (`npm install -g eas-cli`).
- Apple's **Transporter** app installed on your Mac (available from the Mac App Store).
- An Expo account. Log in with `eas login`.

### 2. Configure the Application

Open the `mobileapp/app.json` file. You must configure the `ios.bundleIdentifier`. This is a unique string that identifies your app, typically in reverse domain name notation (e.g., `com.yourfamily.familymonolith`).

Example `app.json` snippet:
```json
{
  "expo": {
    "name": "Family Monolith",
    "slug": "family-monolith-mobile",
    // ... other settings
    "ios": {
      "supportsTablet": true,
      "bundleIdentifier": "com.yourname.familymonolith"
    }
  }
}
```
You will also need to configure your Apple Team ID in `eas.json` after running `eas build:configure`.

### 3. Build the iOS Application

Expo Application Services (EAS) will build a native `.ipa` file for you in the cloud.

Navigate to the mobile app's directory:
```bash
cd mobileapp
```

Start the build process:
```bash
eas build --platform ios
```

EAS will guide you through the process. It will ask you to log in to your Apple Developer account and may create new provisioning profiles and certificates for you if needed. This process can take 30-60 minutes.

### 4. Upload to App Store Connect

Once the build is complete, EAS will provide a link to download the `.ipa` file.

1.  Download the `.ipa` file to your Mac.
2.  Open the **Transporter** application.
3.  Click "Add App" and select the `.ipa` file you downloaded.
4.  Follow the prompts in Transporter to upload the build to App Store Connect. This can also take some time depending on the file size.

### 5. Distribute via TestFlight

1.  Log in to [App Store Connect](https://appstoreconnect.apple.com/).
2.  Go to "My Apps" and select your application.
3.  Navigate to the "TestFlight" tab.
4.  Your uploaded build should appear here after it finishes processing (this can take anywhere from a few minutes to a few hours).
5.  You will need to provide some initial information for Beta App Review if this is the first build.
6.  To add family members, go to "Internal Testing" or "External Testing" on the left sidebar. Add their Apple ID emails as testers. They will receive an email invitation with instructions on how to download the TestFlight app and install your application.
```

## Verification

Read the generated `DEPLOYMENT.md` file. Ensure that the steps are clear, accurate, and provide enough detail for a developer to successfully deploy both applications.
