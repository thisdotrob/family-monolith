# Authentication

This document describes how to authenticate users in your web or mobile application using the GraphQL API.

The authentication system uses JSON Web Tokens (JWT) for securing API requests and refresh tokens for maintaining user sessions.

## Authentication Flow

1.  **Login**: The user logs in with their username and password. The server returns a short-lived JWT (access token) and a long-lived refresh token.
2.  **Authenticated Requests**: The client sends the JWT in the `Authorization` header for all subsequent requests to protected resources.
3.  **Token Refresh**: When the JWT expires, the client uses the refresh token to obtain a new JWT and a new refresh token. This is known as token rotation.
4.  **Logout**: The client explicitly logs the user out by invalidating the refresh token.

## GraphQL Mutations

### API Endpoint

All GraphQL requests should be sent to the following endpoint:

`https://blobfishapp.duckdns.org/graphql`

For mobile applications (like React Native), you must use this full URL. For web applications, you can use a relative path (`/graphql`) because the app is served from the same domain.

### `login`

Use this mutation to authenticate a user and get the initial tokens.

**Mutation:**

```graphql
mutation Login($username: String!, $password: String!) {
  login(input: { username: $username, password: $password }) {
    success
    token
    refreshToken
    errors
  }
}
```

**Example Response:**

```json
{
  "data": {
    "login": {
      "success": true,
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
      "refreshToken": "a-long-and-unique-refresh-token",
      "errors": []
    }
  }
}
```

### `refreshToken`

Use this mutation to get a new JWT and refresh token when the old JWT has expired.

**Mutation:**

```graphql
mutation RefreshToken($refreshToken: String!) {
  refreshToken(input: { refreshToken: $refreshToken }) {
    success
    token
    refreshToken
    errors
  }
}
```

**Example Response:**

```json
{
  "data": {
    "refreshToken": {
      "success": true,
      "token": "a-new-jwt-token",
      "refreshToken": "a-new-refresh-token",
      "errors": []
    }
  }
}
```

### `logout`

Use this mutation to log a user out. This will invalidate the refresh token.

**Mutation:**

```graphql
mutation Logout($refreshToken: String!) {
  logout(input: { refreshToken: $refreshToken }) {
    success
  }
}
```

**Example Response:**

```json
{
  "data": {
    "logout": {
      "success": true
    }
  }
}
```

## React Web Application

A common pattern in React is to create a dedicated "Auth Context" to manage tokens and user state, and a custom hook for making API calls.

### Storing Tokens

It is recommended to store the `token` and `refreshToken` in `localStorage`. This will persist the user's session across browser restarts.

```javascript
// Example of storing tokens after a successful login
const { token, refreshToken } = response.data.login;
localStorage.setItem('token', token);
localStorage.setItem('refreshToken', refreshToken);
```

### Creating an Auth Context

An `AuthContext` can provide authentication state and functions to your entire application.

```javascript
// src/AuthContext.js
import React, { createContext, useState, useContext } from 'react';

const AuthContext = createContext(null);

export const AuthProvider = ({ children }) => {
  const [token, setToken] = useState(localStorage.getItem('token'));

  const saveTokens = (newToken, newRefreshToken) => {
    localStorage.setItem('token', newToken);
    localStorage.setItem('refreshToken', newRefreshToken);
    setToken(newToken);
  };

  const logout = () => {
    localStorage.removeItem('token');
    localStorage.removeItem('refreshToken');
    setToken(null);
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

export const useAuth = () => useContext(AuthContext);
```

You would then wrap your application in this provider:

```javascript
// src/App.js
import { AuthProvider } from './AuthContext';

const App = () => {
  return (
    <AuthProvider>
      {/* The rest of your application */}
    </AuthProvider>
  );
};
```

### Making Authenticated Requests

Create a custom hook or a dedicated API client to handle adding the `Authorization` header. This keeps your component logic clean.

```javascript
// src/api.js
export const fetchGraphQL = async (query, variables = {}) => {
  const token = localStorage.getItem('token');

  const response = await fetch('/graphql', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': token ? `Bearer ${token}` : '',
    },
    body: JSON.stringify({ query, variables }),
  });

  return response.json();
};
```

### Handling Token Expiration

If an API request fails due to an expired token, you should use the `refreshToken` to get a new set of tokens. This logic can be built into your API client. A robust implementation would automatically retry the failed request after a successful token refresh.

## React Native Application

### Storing Tokens

In React Native, you should use `AsyncStorage` for securely storing the tokens on the device.

```javascript
import AsyncStorage from '@react-native-async-storage/async-storage';

// Example of storing tokens after login
const { token, refreshToken } = response.data.login;
await AsyncStorage.setItem('token', token);
await AsyncStorage.setItem('refreshToken', refreshToken);
```

### Making Authenticated Requests

The process is similar to a web application. You'll fetch the token from `AsyncStorage` and include it in the `Authorization` header.

```javascript
import AsyncStorage from '@react-native-async-storage/async-storage';

const token = await AsyncStorage.getItem('token');

fetch('/graphql', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${token}`,
  },
  body: JSON.stringify({ query: '{ me { username } }' }),
});
```

### Handling Token Expiration

The logic for handling token expiration is the same as in a web application. When you receive an authentication error, use the stored `refreshToken` to obtain a new `token` and `refreshToken`.

## Accessing the Database

To access the database for manual queries, you can use the following command. This will open a console to the SQLite database.

```bash
sqlite3 ./blobfishapp.sqlite
```

## Manually Adding a User

To add a user manually, you need to insert a new row into the `users` table. The password is not hashed and is stored in plaintext.

The `users` table has the following columns:

-   `id`: A unique identifier for the user (e.g., a UUID).
-   `username`: The user's username.
-   `password`: The user's password in plaintext.
-   `first_name`: The user's first name.

Here is an example of how to insert a new user:

```sql
INSERT INTO users (id, username, password, first_name)
VALUES ('a-unique-id', 'newuser', 'password123', 'John');
```
