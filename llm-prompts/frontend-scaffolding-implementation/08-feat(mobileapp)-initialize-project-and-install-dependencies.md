You are an expert mobile software engineer. Your task is to initialize a new React Native project using Expo and install its core dependencies.

**Commit Title:** `feat(mobileapp): initialize project and install dependencies`

## 1. Project Initialization

From the `monolith-frontend` directory, create a new Expo project named `mobileapp`. When prompted, choose the "Blank" template.

```bash
npx create-expo-app mobileapp
```

Navigate into the new directory:

```bash
cd mobileapp
```

## 2. Install Dependencies

Install the necessary dependencies for UI components (React Native Paper), GraphQL (Apollo), and secure storage (`AsyncStorage`).

```bash
npx expo install react-native-paper @react-native-async-storage/async-storage @apollo/client graphql
```
Using `npx expo install` is important as it ensures compatible versions of the native libraries are installed.

## 3. Configure React Native Paper

To use React Native Paper, you need to wrap your application in its `PaperProvider`.

**`App.tsx`:**
Replace the entire file content with the following setup.
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { View, Text, StyleSheet } from 'react-native';

export default function App() {
  return (
    <PaperProvider>
      <View style={styles.container}>
        <Text>Mobile App Placeholder</Text>
      </View>
    </PaperProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
```

## 4. Verification

After completing these steps, run the development server:

```bash
npx expo start
```

This will open a terminal interface with a QR code. Scan the QR code with the Expo Go app on your iOS or Android device.

The app should load and display the text "Mobile App Placeholder" on a white background. This confirms the project is set up correctly and the core dependencies are in place.
