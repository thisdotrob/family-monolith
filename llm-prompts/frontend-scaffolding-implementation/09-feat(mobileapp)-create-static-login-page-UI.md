You are an expert mobile software engineer. Building on the initialized mobile app, your task is to create the static user interface for the login page using React Native Paper components.

**Commit Title:** `feat(mobileapp): create static login page UI`

## 1. Create the Login Page Component

Create a new directory `src/pages` and a new file `src/pages/LoginPage.tsx`. This component will contain the login form, styled with React Native Paper.

**`src/pages/LoginPage.tsx`:**
```tsx
import React from 'react';
import { View, StyleSheet } from 'react-native';
import { TextInput, Button, Text } from 'react-native-paper';

const LoginPage = () => {
  return (
    <View style={styles.container}>
      <View style={styles.content}>
        <Text variant="headlineLarge" style={styles.title}>Login</Text>
        
        {/* This View will be used for success/failure messages */}
        <View style={styles.messageContainer}></View>

        <TextInput
          label="Username"
          mode="outlined"
          style={styles.input}
          autoCapitalize="none"
        />
        <TextInput
          label="Password"
          mode="outlined"
          style={styles.input}
          secureTextEntry
        />
        <Button
          mode="contained"
          style={styles.button}
        >
          Sign In
        </Button>
      </View>
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    backgroundColor: '#f5f5f5',
  },
  content: {
    padding: 20,
  },
  title: {
    textAlign: 'center',
    marginBottom: 24,
  },
  messageContainer: {
    marginBottom: 16,
    minHeight: 20,
  },
  input: {
    marginBottom: 16,
  },
  button: {
    marginTop: 8,
  },
});

export default LoginPage;
```

## 2. Render the Login Page

Update `App.tsx` to render the newly created `LoginPage`. Note that we are removing the placeholder styles and wrapping the app in a `SafeAreaView` for better layout on notched devices, although for this centered screen it's less critical.

**`App.tsx`:**
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet } from 'react-native';
import LoginPage from './src/pages/LoginPage';

export default function App() {
  return (
    <PaperProvider>
      <SafeAreaView style={styles.container}>
        <LoginPage />
      </SafeAreaView>
    </PaperProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

## 3. Verification

Run the application (`npx expo start`) and open it in the Expo Go app.

The screen should now display a login form with "Username" and "Password" fields and a "Sign In" button, all styled according to Material Design principles via React Native Paper.
