You are an expert software engineer. This is a complex but crucial step. Your task is to implement the automatic token refresh logic using an Apollo Link.

**Commit Title:** `feat(webapp): implement automatic token refresh link`

## 1. Add a Refreshing State to AuthContext

First, we need a way to signal to the UI that a token refresh is in progress.

**`src/contexts/AuthContext.tsx`:**
Update the `AuthContextType` and the `AuthProvider` to include an `isRefreshing` state.
```tsx
// ... imports

interface AuthContextType {
  token: string | null;
  isRefreshing: boolean; // Add this
  saveTokens: (newToken: string, newRefreshToken: string) => void;
  logout: () => void;
  setIsRefreshing: (isRefreshing: boolean) => void; // Add this
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('token'));
  const [isRefreshing, setIsRefreshing] = useState(false); // Add this

  // ... saveTokens is unchanged

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refreshToken');
    setToken(null);
    // We will clear the cache properly in the apollo.ts file now
  };

  const authContextValue = {
    token,
    isRefreshing, // Add this
    saveTokens,
    logout,
    setIsRefreshing, // Add this
  };

  // ... return statement is unchanged
};

// ... useAuth is unchanged
```

## 2. Create a Global Loading Overlay

Create a component to block the UI during the refresh.

**`src/components/GlobalLoading.tsx`:**
Create a new directory and file.
```tsx
import React from 'react';
import { useAuth } from '../contexts/AuthContext';

const GlobalLoading = () => {
  const { isRefreshing } = useAuth();

  if (!isRefreshing) {
    return null;
  }

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 z-50 flex items-center justify-center">
      <div className="bg-white p-6 rounded-lg shadow-xl">
        <p className="text-lg font-semibold">Refreshing session...</p>
      </div>
    </div>
  );
};

export default GlobalLoading;
```

**`src/App.tsx`:**
Render this component in your app.
```tsx
import { useAuth } from './contexts/AuthContext';
import HomePage from './pages/HomePage';
import LoginPage from './pages/LoginPage';
import GlobalLoading from './components/GlobalLoading';

function App() {
  const { token } = useAuth();

  return (
    <>
      <GlobalLoading />
      {token ? <HomePage /> : <LoginPage />}
    </>
  );
}

export default App;
```

## 3. Implement the Token Refresh Link

This is the core of the task. Update `src/api/apollo.ts` to handle GraphQL errors, attempt a token refresh, and retry the failed request.

**`src/graphql/mutations.ts`:**
First, add the `refreshToken` mutation.
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
This is a significant update. We will use `onError` from `@apollo/client/link/error` to catch errors.
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
import { REFRESH_TOKEN_MUTATION } from '../graphql/mutations';

// A separate client for the refresh token mutation to avoid link loops
const refreshClient = new ApolloClient({
  uri: '/graphql',
  cache: new InMemoryCache(),
});

const httpLink = createHttpLink({
  uri: '/graphql',
});

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('token');
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
  onError(({ graphQLErrors, networkError, operation, forward }) => {
    if (graphQLErrors) {
      for (let err of graphQLErrors) {
        // Check for a specific error message or code that indicates an expired token
        if (err.message.includes('Unauthorized')) {
          const refreshToken = localStorage.getItem('refreshToken');
          if (!refreshToken) {
            logout();
            return;
          }

          setIsRefreshing(true);

          refreshClient
            .mutate({
              mutation: REFRESH_TOKEN_MUTATION,
              variables: { refreshToken },
            })
            .then(({ data }) => {
              const newTokens = data?.refreshToken;
              if (newTokens?.success) {
                localStorage.setItem('token', newTokens.token);
                localStorage.setItem('refreshToken', newTokens.refreshToken);

                // Retry the failed request with the new token
                const oldHeaders = operation.getContext().headers;
                operation.setContext({
                  headers: {
                    ...oldHeaders,
                    authorization: `Bearer ${newTokens.token}`,
                  },
                });
                return forward(operation);
              } else {
                logout();
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

    if (networkError) console.log(`[Network error]: ${networkError}`);
  });

// The main client factory
export const createApolloClient = (
  setIsRefreshing: (isRefreshing: boolean) => void,
  baseLogout: () => void
) => {
  const logout = async () => {
    baseLogout();
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

We now need to pass the context functions to our client factory.

**`src/main.tsx`:**
This needs to be refactored slightly so we can access the `useAuth` hook's functions.
```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import { ApolloProvider, ApolloClient, InMemoryCache } from '@apollo/client';
import { AuthProvider, useAuth } from './contexts/AuthContext';
import App from './App.tsx';
import './index.css';
import { createApolloClient } from './api/apollo.ts';

const Main = () => {
  // We need to be inside AuthProvider to use the useAuth hook
  const { setIsRefreshing, logout } = useAuth();
  const client = createApolloClient(setIsRefreshing, logout);

  return (
    <ApolloProvider client={client}>
      <App />
    </ApolloProvider>
  );
};

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthProvider>
      <Main />
    </AuthProvider>
  </React.StrictMode>
);
```

## 5. Verification

This is the most difficult part to test.

1.  **Simulate Expired Token**:
    *   Log in to the application normally.
    *   Go into your browser's developer tools.
    *   In `localStorage`, manually change the `token` to an invalid value (e.g., `eyJhb...invalid`).
    *   Refresh the `HomePage`.
2.  **Observe Behavior**:
    *   The `me` query on the `HomePage` should fail with an "Unauthorized" error.
    *   The UI should be blocked by the "Refreshing session..." overlay.
    *   The application should send a `refreshToken` mutation in the background.
    *   If the refresh is successful, the overlay should disappear, the `me` query should be retried (this time with a valid token), and the page should load correctly.
    *   If the `refreshToken` is also invalid or expired, you should be logged out and redirected to the `LoginPage`.
