You are an expert software engineer. Your task is to create an authentication context to manage user state and persist tokens to `localStorage`.

**Commit Title:** `feat(webapp): implement AuthContext and token storage`

## 1. Create the AuthContext

Create a new directory `src/contexts` and a file `src/contexts/AuthContext.tsx`. This context will provide authentication state and functions to the entire application.

**`src/contexts/AuthContext.tsx`:**
```tsx
import React, { createContext, useState, useContext, ReactNode } from 'react';

interface AuthContextType {
  token: string | null;
  saveTokens: (newToken: string, newRefreshToken: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | null>(null);

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(() => localStorage.getItem('token'));

  const saveTokens = (newToken: string, newRefreshToken: string) => {
    localStorage.setItem('token', newToken);
    localStorage.setItem('refreshToken', newRefreshToken);
    setToken(newToken);
  };

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refreshToken');
    setToken(null);
    // In a real app, you'd also want to clear the Apollo Client cache here.
    // We will add this in a later step.
  };

  const authContextValue = {
    token,
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

Wrap the `ApolloProvider` with the new `AuthProvider` in `src/main.tsx` to make the context available everywhere.

**`src/main.tsx`:**
```tsx
import React from 'react'
import ReactDOM from 'react-dom/client'
import { ApolloProvider } from '@apollo/client'
import { AuthProvider } from './contexts/AuthContext'
import App from './App.tsx'
import './index.css'
import client from './api/apollo.ts'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <AuthProvider>
      <ApolloProvider client={client}>
        <App />
      </ApolloProvider>
    </AuthProvider>
  </React.StrictMode>,
)
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

Finally, update the `onCompleted` handler for the mutation to call `saveTokens`:
```tsx
// inside the useMutation hook options
onCompleted: (data) => {
  if (data.login.success) {
    setMessage({ text: 'Login successful!', type: 'success' });
    saveTokens(data.login.token, data.login.refreshToken);
  } else {
    // ... same as before
  }
},
```

## 4. Verification

1.  Run the application (`npm run dev`).
2.  Perform a successful login.
3.  Open your browser's developer tools and inspect the `localStorage`.
4.  You should see two keys, `token` and `refreshToken`, populated with the values received from the API.
5.  Refresh the page. The application state should be maintained (though there's no visible change yet, the token will be loaded from `localStorage` into the context).
