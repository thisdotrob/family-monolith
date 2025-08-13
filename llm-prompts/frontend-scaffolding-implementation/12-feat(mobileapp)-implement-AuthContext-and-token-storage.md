You are an expert mobile software engineer. Your task is to create an authentication context for the mobile app to manage user state and persist tokens to `AsyncStorage`.

**Commit Title:** `feat(mobileapp): implement AuthContext and token storage`

## 1. Create the AuthContext

Create a new directory `src/contexts` and a file `src/contexts/AuthContext.tsx`. This context will be very similar to the web app's, but will use `AsyncStorage`.

**`src/contexts/AuthContext.tsx`:**
```tsx
import React, { createContext, useState, useContext, ReactNode, useEffect } from 'react';
import AsyncStorage from '@react-native-async-storage/async-storage';

interface AuthContextType {
  token: string | null;
  isLoading: boolean; // To handle async storage loading
  saveTokens: (newToken: string, newRefreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const loadToken = async () => {
      try {
        const storedToken = await AsyncStorage.getItem('token');
        setToken(storedToken);
      } catch (e) {
        console.error('Failed to load token from storage', e);
      } finally {
        setIsLoading(false);
      }
    };

    loadToken();
  }, []);

  const saveTokens = async (newToken: string, newRefreshToken: string) => {
    try {
      await AsyncStorage.setItem('token', newToken);
      await AsyncStorage.setItem('refreshToken', newRefreshToken);
      setToken(newToken);
    } catch (e) {
      console.error('Failed to save tokens to storage', e);
    }
  };

  const logout = async () => {
    try {
      await AsyncStorage.removeItem('token');
      await AsyncStorage.removeItem('refreshToken');
      setToken(null);
    } catch (e) {
      console.error('Failed to remove tokens from storage', e);
    }
  };

  const authContextValue = {
    token,
    isLoading,
    saveTokens,
    logout,
  };

  return (
    <AuthContext.Provider value={authContextValue}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (!context) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
```

## 2. Provide the AuthContext to the Application

Wrap the existing providers with the new `AuthProvider` in `App.tsx`.

**`App.tsx`:**
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet } from 'react-native';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider } from './src/contexts/AuthContext';
import LoginPage from './src/pages/LoginPage';
import client from './src/api/apollo';

export default function App() {
  return (
    <AuthProvider>
      <ApolloProvider client={client}>
        <PaperProvider>
          <SafeAreaView style={styles.container}>
            <LoginPage />
          </SafeAreaView>
        </PaperProvider>
      </ApolloProvider>
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

## 3. Integrate AuthContext with the Login Page

Update `src/pages/LoginPage.tsx` to use the `useAuth` hook and save the tokens upon a successful login.

**`src/pages/LoginPage.tsx` (partial changes):**
First, add the `useAuth` import:
```tsx
import { useAuth } from '../contexts/AuthContext';
```

Next, get the `saveTokens` function from the context:
```tsx
// inside the LoginPage component
const { saveTokens } = useAuth();
```

Finally, update the `onCompleted` handler for the mutation to call `saveTokens`. Note that it's an `async` call.
```tsx
// inside the useMutation hook options
onCompleted: async (data) => {
  if (data.login.success) {
    setMessage({ text: 'Login successful!', type: 'success' });
    await saveTokens(data.login.token, data.login.refreshToken);
  } else {
    // ... same as before
  }
},
```

## 4. Verification

1.  Run the application.
2.  Perform a successful login.
3.  The tokens should be saved to `AsyncStorage`. While you can't inspect this directly like in a browser, the app should function correctly in the next step.
4.  To verify persistence, fully close the Expo Go app and reopen it. The user should remain logged in (which we will build the UI for in the next step). The `isLoading` state in the context will handle the initial check.
