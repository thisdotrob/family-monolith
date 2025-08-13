You are an expert mobile software engineer. Your final functional task is to implement the automatic token refresh logic for the React Native application.

**Commit Title:** `feat(mobileapp): implement automatic token refresh link`

## 1. Add Refreshing State to AuthContext

Update `src/contexts/AuthContext.tsx` to include an `isRefreshing` state and a way to set it.

**`src/contexts/AuthContext.tsx` (partial changes):**
```tsx
// ... imports

interface AuthContextType {
  token: string | null;
  isLoading: boolean;
  isRefreshing: boolean; // Add this
  saveTokens: (newToken: string, newRefreshToken: string) => Promise<void>;
  logout: () => Promise<void>;
  setIsRefreshing: (isRefreshing: boolean) => void; // Add this
}

// ... AuthContext

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [isRefreshing, setIsRefreshing] = useState(false); // Add this

  // ... useEffect and saveTokens are unchanged

  const logout = async () => {
    try {
      await AsyncStorage.multiRemove(['token', 'refreshToken']);
      setToken(null);
    } catch (e) {
      console.error('Failed to remove tokens from storage', e);
    }
  };

  const authContextValue = {
    token,
    isLoading,
    isRefreshing, // Add this
    saveTokens,
    logout,
    setIsRefreshing, // Add this
  };

  return (
    <AuthContext.Provider value={authContextValue}>
      {children}
    </AuthContext.Provider>
  );
};

// ... useAuth is unchanged
```

## 2. Create a Global Loading Overlay

Create a component at `src/components/GlobalLoading.tsx` to block the UI during the refresh.

**`src/components/GlobalLoading.tsx`:**
```tsx
import React from 'react';
import { View, Modal, StyleSheet } from 'react-native';
import { ActivityIndicator, Text } from 'react-native-paper';
import { useAuth } from '../contexts/AuthContext';

const GlobalLoading = () => {
  const { isRefreshing } = useAuth();

  return (
    <Modal visible={isRefreshing} transparent={true} animationType="fade">
      <View style={styles.container}>
        <View style={styles.content}>
          <ActivityIndicator animating={true} size="large" />
          <Text style={styles.text}>Refreshing session...</Text>
        </View>
      </View>
    </Modal>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
  },
  content: {
    backgroundColor: 'white',
    padding: 20,
    borderRadius: 10,
    alignItems: 'center',
  },
  text: {
    marginTop: 16,
    fontSize: 16,
  },
});

export default GlobalLoading;
```

**`App.tsx`:**
Render this component in your app.
```tsx
// ... imports
import Router from './src/components/Router';
import GlobalLoading from './src/components/GlobalLoading';

export default function App() {
  return (
    <AuthProvider>
      <ApolloProvider client={client}>
        <PaperProvider>
          <SafeAreaView style={styles.container}>
            <Router />
            <GlobalLoading />
          </SafeAreaView>
        </PaperProvider>
      </ApolloProvider>
    </AuthProvider>
  );
}
// ... styles
```

## 3. Implement the Token Refresh Link

Update `src/api/apollo.ts` to handle token refresh. This will be very similar to the web implementation.

**`src/graphql/mutations.ts`:**
Add the `refreshToken` mutation.
```ts
// ... LOGIN_MUTATION

export const REFRESH_TOKEN_MUTATION = gql`
  mutation RefreshToken($refreshToken: String!) {
    refreshToken(input: { refreshToken: $refreshToken }) {
      success
      token
      refreshToken
      errors
    }
  }
`;
```

**`src/api/apollo.ts`:**
This is a significant update to create the error handling link.
```ts
import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  from,
  ApolloLink,
} from '@apollo/client';
import { setContext } from '@apollo/client/link/context';
import { onError } from '@apollo/client/link/error';
import AsyncStorage from '@react-native-async-storage/async-storage';
import { REFRESH_TOKEN_MUTATION } from '../graphql/mutations';

const API_URL = 'https://blobfishapp.duckdns.org/graphql';

const refreshClient = new ApolloClient({
  uri: API_URL,
  cache: new InMemoryCache(),
});

const httpLink = createHttpLink({
  uri: API_URL,
});

const authLink = setContext(async (_, { headers }) => {
  const token = await AsyncStorage.getItem('token');
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  };
});

const errorLink = (
  setIsRefreshing: (isRefreshing: boolean) => void,
  logout: () => Promise<void>
): ApolloLink =>
  onError(({ graphQLErrors, operation, forward }) => {
    if (graphQLErrors) {
      for (let err of graphQLErrors) {
        if (err.message.includes('Unauthorized')) {
          setIsRefreshing(true);
          AsyncStorage.getItem('refreshToken')
            .then((refreshToken) => {
              if (!refreshToken) {
                throw new Error('No refresh token');
              }
              return refreshClient.mutate({
                mutation: REFRESH_TOKEN_MUTATION,
                variables: { refreshToken },
              });
            })
            .then(({ data }) => {
              const newTokens = data?.refreshToken;
              if (newTokens?.success) {
                AsyncStorage.setItem('token', newTokens.token);
                AsyncStorage.setItem('refreshToken', newTokens.refreshToken);
                
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: { ...oldHeaders, authorization: `Bearer ${newTokens.token}` },
                });
                return forward(operation);
              } else {
                throw new Error('Refresh failed');
              }
            })
            .catch(() => {
              logout();
            })
            .finally(() => {
              setIsRefreshing(false);
            });
        }
      }
    }
  });

export const createApolloClient = (
  setIsRefreshing: (isRefreshing: boolean) => void,
  baseLogout: () => Promise<void>
) => {
  const logout = async () => {
    await baseLogout();
    await client.clearStore();
  };

  const client = new ApolloClient({
    link: from([errorLink(setIsRefreshing, logout), authLink, httpLink]),
    cache: new InMemoryCache(),
  });

  return client;
};
```

## 4. Update Application Entrypoint

Refactor `App.tsx` to use the new client factory.

**`App.tsx`:**
```tsx
import React from 'react';
import { PaperProvider } from 'react-native-paper';
import { SafeAreaView, StyleSheet } from 'react-native';
import { ApolloProvider } from '@apollo/client';
import { AuthProvider, useAuth } from './src/contexts/AuthContext';
import { createApolloClient } from './src/api/apollo';
import Router from './src/components/Router';
import GlobalLoading from './src/components/GlobalLoading';

const Main = () => {
  const { setIsRefreshing, logout } = useAuth();
  const client = createApolloClient(setIsRefreshing, logout);

  return (
    <ApolloProvider client={client}>
      <PaperProvider>
        <SafeAreaView style={styles.container}>
          <Router />
          <GlobalLoading />
        </SafeAreaView>
      </PaperProvider>
    </ApolloProvider>
  );
};

export default function App() {
  return (
    <AuthProvider>
      <Main />
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
  },
});
```

## 5. Verification

1.  **Simulate Expired Token**: Log in normally. You'll need a way to invalidate the token. One way is to use a proxy to intercept and modify the request, but a simpler way is to have a "test" button on the `HomePage` that manually sets an invalid token in `AsyncStorage` and then triggers a refetch of the `me` query.
2.  **Observe Behavior**:
    *   The `me` query should fail.
    *   The "Refreshing session..." modal should appear.
    *   A `refreshToken` mutation should be sent.
    *   If successful, the modal disappears, and the `HomePage` loads.
    *   If it fails, the user is logged out and returned to the `LoginPage`.
